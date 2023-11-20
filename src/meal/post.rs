use axum::{extract::State, Json};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::QueryBuilder;
use thiserror::Error;

use crate::{
    get_missing_items,
    models::{Meal, MealDish, NewMeal},
    server::{ServerResponse, ServerResponseResult},
    state::AppState,
};

#[derive(Deserialize, JsonSchema)]
pub struct PostMealDish {
    weight: i64,
    dish_id: i64,
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
    pub dishes: Vec<PostMealDish>,
}

#[derive(Serialize, JsonSchema)]
pub struct PostMealResult {
    meal: Meal,
    meal_dishes: Vec<MealDish>,
}

#[derive(Error, Debug)]
enum PostMealError {
    #[error("The following dishes don't exits: {0:?}")]
    UnknownDishId(Vec<i64>),
}

pub async fn post_meal(
    State(AppState { connection }): State<AppState>,
    Json(post_meal): Json<PostMealBody>,
) -> ServerResponseResult<PostMealResult> {
    if !post_meal.dishes.is_empty() {
        let unknown_ingredients = {
            let ids = post_meal
                .dishes
                .iter()
                .map(|item| item.dish_id.to_string())
                .collect::<Vec<String>>()
                .join(", ");

            let dishes_in_database = sqlx::QueryBuilder::new("SELECT id FROM Dish WHERE id IN (")
                .push(ids)
                .push(")")
                .build_query_scalar::<i64>()
                .fetch_all(&connection)
                .await?;

            get_missing_items(
                dishes_in_database,
                post_meal.dishes.iter().map(|d| d.dish_id),
            )
        };

        if !unknown_ingredients.is_empty() {
            return Err(PostMealError::UnknownDishId(unknown_ingredients))?;
        }
    }

    let transaction = connection.begin().await?;

    let meal = sqlx::query_as!(
        Meal,
        r#"INSERT INTO Meal (
            eat_date,
            duration,
            description
        ) VALUES (?, ?, ?) RETURNING *;"#,
        post_meal.eat_date,
        post_meal.duration,
        post_meal.description
    )
    .fetch_one(&connection)
    .await?;

    let meal_dishes = if !post_meal.dishes.is_empty() {
        QueryBuilder::new("INSERT INTO MealDish (meal_id, dish_id, weight)")
            .push_values(post_meal.dishes, |mut b, dish| {
                b.push_bind(meal.id)
                    .push_bind(dish.dish_id)
                    .push_bind(dish.weight);
            })
            .push("RETURNING *")
            .build_query_as::<MealDish>()
            .fetch_all(&connection)
            .await?
    } else {
        vec![]
    };

    transaction.commit().await?;

    Ok(ServerResponse::success(PostMealResult { meal, meal_dishes }).json())
}
