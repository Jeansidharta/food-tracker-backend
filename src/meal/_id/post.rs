use axum::{
    extract::{Path, State},
    Json,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::QueryBuilder;
use thiserror::Error;

use crate::{
    get_missing_items,
    models::{Meal, NewMeal},
    server::{ServerResponse, ServerResponseResult},
    state::AppState,
};

struct Component {
    id: i64,
    weight: i64,
}

#[derive(Deserialize, JsonSchema)]
pub struct MealId {
    meal_id: i64,
}

#[derive(Serialize, sqlx::FromRow, JsonSchema)]
pub struct MealComponent {
    pub creation_date: i64,
    pub id: i64,
    pub meal_id: i64,
    pub weight: i64,
}

#[derive(Deserialize, JsonSchema)]
pub struct PostMealComponent {
    weight: i64,
    dish_id: Option<i64>,
    ingredient_id: Option<i64>,
}

impl From<PostMealBody> for NewMeal {
    fn from(val: PostMealBody) -> Self {
        NewMeal {
            eat_date: val.eat_date,
            duration: val.duration,
            description: val.description,
        }
    }
}

#[derive(Deserialize, JsonSchema)]
pub struct PostMealBody {
    pub eat_date: Option<i64>,
    pub duration: Option<i64>,
    pub description: Option<String>,
    pub components: Vec<PostMealComponent>,
}

#[derive(Serialize, JsonSchema)]
pub struct PostMealResult {
    meal: Meal,
    meal_dishes: Vec<MealComponent>,
    meal_ingredients: Vec<MealComponent>,
}

#[derive(Error, Debug)]
enum PostMealError {
    #[error("The following dishes don't exits: {0:?}")]
    UnknownDishId(Vec<i64>),
    #[error("In one of the components provided, there was no dish_id and no ingredient_id")]
    NoDishIdProvided,
    #[error("In one of the components provided, both dish_id and ingredient_id were provided")]
    DishIdAndIngredientIdProvided,
}

enum ComponentType {
    Ingredient,
    Dish,
}

impl ComponentType {
    fn to_string(self) -> &'static str {
        match self {
            ComponentType::Ingredient => "Ingredient",
            ComponentType::Dish => "Dish",
        }
    }
}

async fn check_missing_component(
    connection: &sqlx::Pool<sqlx::Sqlite>,
    component_type: ComponentType,
    component_ids: &Vec<Component>,
) -> anyhow::Result<()> {
    if component_ids.is_empty() {
        return Ok(());
    }

    let unknown_parts = {
        let ids = component_ids
            .iter()
            .map(|item| item.id.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        let dishes_in_database = sqlx::QueryBuilder::new("SELECT id FROM ")
            .push(component_type.to_string())
            .push(" Dish WHERE id IN (")
            .push(ids)
            .push(")")
            .build_query_scalar::<i64>()
            .fetch_all(connection)
            .await?;

        get_missing_items(dishes_in_database, component_ids.iter().map(|i| i.id))
    };

    if !unknown_parts.is_empty() {
        return Err(PostMealError::UnknownDishId(unknown_parts))?;
    }
    Ok(())
}

pub async fn post_meal(
    State(AppState { connection }): State<AppState>,
    Path(MealId { meal_id: id }): Path<MealId>,
    Json(post_meal): Json<PostMealBody>,
) -> ServerResponseResult<PostMealResult> {
    let (dishes, ingredients) = post_meal.components.into_iter().try_fold(
        (vec![], vec![]),
        |(mut dishAcc, mut ingredientAcc),
         PostMealComponent {
             weight,
             dish_id,
             ingredient_id,
         }| {
            match (dish_id, ingredient_id) {
                (None, None) => return Err(PostMealError::NoDishIdProvided),
                (None, Some(id)) => ingredientAcc.push(Component { id, weight }),
                (Some(id), None) => dishAcc.push(Component { id, weight }),
                (Some(_), Some(_)) => return Err(PostMealError::DishIdAndIngredientIdProvided),
            }
            Ok((dishAcc, ingredientAcc))
        },
    )?;

    futures::try_join!(
        check_missing_component(&connection, ComponentType::Dish, &dishes),
        check_missing_component(&connection, ComponentType::Ingredient, &ingredients)
    )?;

    let transaction = connection.begin().await?;

    let meal = sqlx::query_as!(
        Meal,
        r#"UPDATE Meal SET
            eat_date = ?,
            duration = ?,
            description = ?
        WHERE Meal.id = ?
        RETURNING id as 'id!', creation_date, eat_date, duration, description;"#,
        post_meal.eat_date,
        post_meal.duration,
        post_meal.description,
        id
    )
    .fetch_one(&connection)
    .await?;

    sqlx::query!(
        r#"DELETE FROM MealIngredient WHERE meal_id = ?;
        DELETE FROM MealDish WHERE meal_id = ?; "#,
        id,
        id
    )
    .execute(&connection)
    .await?;

    let meal_ingredients = if !ingredients.is_empty() {
        QueryBuilder::new("INSERT INTO MealIngredient (meal_id, ingredient_id, weight)")
            .push_values(ingredients, |mut b, Component { id, weight }| {
                b.push_bind(meal.id).push_bind(id).push_bind(weight);
            })
            .push(
                r#" ON CONFLICT DO UPDATE SET weight = MealIngredient.weight + excluded.weight
                RETURNING meal_id, ingredient_id as id, weight, creation_date"#,
            )
            .build_query_as::<MealComponent>()
            .fetch_all(&connection)
            .await?
    } else {
        vec![]
    };
    let meal_dishes = if !dishes.is_empty() {
        QueryBuilder::new("INSERT INTO MealDish (meal_id, dish_id, weight)")
            .push_values(dishes, |mut b, Component { id, weight }| {
                b.push_bind(meal.id).push_bind(id).push_bind(weight);
            })
            .push(
                r#" ON CONFLICT DO UPDATE SET weight = MealDish.weight + excluded.weight
                RETURNING meal_id, dish_id as id, weight, creation_date"#,
            )
            .build_query_as::<MealComponent>()
            .fetch_all(&connection)
            .await?
    } else {
        vec![]
    };

    transaction.commit().await?;

    Ok(ServerResponse::success(PostMealResult {
        meal,
        meal_dishes,
        meal_ingredients,
    })
    .json())
}
