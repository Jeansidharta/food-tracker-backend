use axum::extract::{Path, State};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    models::Meal,
    server::{ServerResponse, ServerResponseResult},
    state::AppState,
};

#[derive(Serialize, JsonSchema)]
pub struct MealComponent {
    weight: i64,
    name: Option<String>,
    id: i64,
    kcal_100g: Option<i64>,
}

#[derive(Serialize, JsonSchema)]
pub struct GetMealResponse {
    meal: Meal,
    dishes: Vec<MealComponent>,
    ingredients: Vec<MealComponent>,
}

#[derive(Error, Debug)]
enum GetMeal {
    #[error("Could not find meal with id \"{0}\"")]
    MealNotFound(i64),
}

#[derive(Deserialize, JsonSchema)]
pub struct MealId {
    meal_id: i64,
}

async fn get_meal_table(
    connection: &sqlx::Pool<sqlx::Sqlite>,
    meal_id: i64,
) -> anyhow::Result<Meal> {
    Ok(sqlx::query_as!(
        Meal,
        r#"
        SELECT
            id,
            creation_date,
            duration,
            description,
            eat_date
        FROM Meal
        WHERE id = ?;"#,
        meal_id
    )
    .fetch_optional(connection)
    .await?
    .ok_or(GetMeal::MealNotFound(meal_id))?)
}

async fn get_meal_ingredients_table(
    connection: &sqlx::Pool<sqlx::Sqlite>,
    meal_id: i64,
) -> anyhow::Result<Vec<MealComponent>> {
    Ok(sqlx::query_as!(
        MealComponent,
        r#"
        SELECT 
            MealIngredient.weight,
            Ingredient.name as name,
            Ingredient.id as id,
            kcal_100g
        FROM Meal
            JOIN MealIngredient ON Meal.id = MealIngredient.meal_id
            JOIN Ingredient ON MealIngredient.ingredient_id = Ingredient.id
            LEFT JOIN IngredientProperties ON IngredientProperties.ingredient_id = Ingredient.id
        WHERE Meal.id = ?;
        "#,
        meal_id
    )
    .fetch_all(connection)
    .await?)
}

async fn get_meal_dishes_table(
    connection: &sqlx::Pool<sqlx::Sqlite>,
    meal_id: i64,
) -> anyhow::Result<Vec<MealComponent>> {
    Ok(sqlx::query_as!(
        MealComponent,
        r#"
        SELECT 
            MealDish.weight,
            Dish.name as name,
            Dish.id as id,
            (
                SELECT (
                    TOTAL (kcal_100g * weight) / (
                        CASE WHEN Dish.total_weight IS NULL THEN
                            SUM(weight)
                        ELSE
                            Dish.total_weight
                        END
                    )
                ) as kcal_100g
                FROM DishIngredient
                JOIN Ingredient on Ingredient.id = DishIngredient.ingredient_id
                JOIN IngredientProperties on Ingredient.id = IngredientProperties.ingredient_id
                WHERE DishIngredient.dish_id = Dish.id
            ) as 'kcal_100g: i64'
        FROM Meal
            JOIN MealDish ON Meal.id = MealDish.meal_id
            JOIN Dish ON MealDish.dish_id = Dish.id
        WHERE Meal.id = ?;
        "#,
        meal_id
    )
    .fetch_all(connection)
    .await?)
}

pub async fn get_meal(
    State(AppState { connection }): State<AppState>,
    Path(MealId { meal_id }): Path<MealId>,
) -> ServerResponseResult<GetMealResponse> {
    let (meal, dishes, ingredients) = futures::try_join!(
        get_meal_table(&connection, meal_id),
        get_meal_dishes_table(&connection, meal_id),
        get_meal_ingredients_table(&connection, meal_id),
    )?;

    Ok(ServerResponse::success(GetMealResponse {
        meal,
        dishes,
        ingredients,
    })
    .json())
}
