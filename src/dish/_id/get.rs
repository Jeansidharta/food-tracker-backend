use axum::extract::{Path, State};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    models::Dish,
    server::{ServerResponse, ServerResponseResult},
    state::AppState,
};

#[derive(Serialize, JsonSchema)]
pub struct UsedAt {
    meal_description: Option<String>,
    meal_id: i64,
    eat_date: Option<i64>,
    weight: Option<i64>,
}

#[derive(Serialize, JsonSchema)]
pub struct AddedIngredient {
    addition_date: i64,
    weight: i64,
    ingredient_name: String,
    ingredient_id: i64,
    kcal_100g: Option<i64>,
}

#[derive(Serialize, JsonSchema)]
pub struct GetDishResponse {
    dish: Dish,
    added_ingredients: Vec<AddedIngredient>,
    used_at: Vec<UsedAt>,
}

#[derive(Error, Debug)]
enum GetDishError {
    #[error("Could not find dish with id \"{0}\"")]
    DishNotFound(i64),
}

#[derive(Deserialize, JsonSchema)]
pub struct DishId {
    dish_id: i64,
}

pub async fn get_dish(
    State(AppState { connection }): State<AppState>,
    Path(DishId { dish_id: id }): Path<DishId>,
) -> ServerResponseResult<GetDishResponse> {
    let dish = sqlx::query_as!(
        Dish,
        r#"
        SELECT 
            id,
            creation_date,
            name,
            prep_date,
            total_weight,
            is_finished
        FROM Dish
        WHERE Dish.id = ?"#,
        id
    )
    .fetch_optional(&connection)
    .await?
    .ok_or(GetDishError::DishNotFound(id))?;

    let added_ingredients = sqlx::query_as!(
        AddedIngredient,
        r#"
        SELECT 
            weight,
            ingredient.name as ingredient_name,
            DishIngredient.creation_date as addition_date,
            DishIngredient.ingredient_id,
            kcal_100g
        FROM Dish
            JOIN DishIngredient ON Dish.id = DishIngredient.dish_id
            JOIN Ingredient on DishIngredient.ingredient_id = Ingredient.id
            JOIN IngredientProperties on IngredientProperties.ingredient_id = Ingredient.id
        WHERE Dish.id = ?;
        "#,
        id
    )
    .fetch_all(&connection)
    .await?
    .into_iter()
    .collect();

    let used_at = sqlx::query_as!(
        UsedAt,
        r#"
        SELECT
            MealDish.meal_id,
            MealDish.weight as weight,
            Meal.eat_date,
            Meal.description as meal_description
        FROM MealDish JOIN Meal ON Meal.id = MealDish.meal_id
        WHERE dish_id = ?
        ORDER BY Meal.eat_date DESC NULLS FIRST;
        "#,
        id
    )
    .fetch_all(&connection)
    .await?;
    Ok(ServerResponse::success(GetDishResponse {
        dish,
        added_ingredients,
        used_at,
    })
    .json())
}
