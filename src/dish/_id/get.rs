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
pub struct AddedIngredient {
    addition_date: i64,
    weight: i64,
    ingredient_name: String,
    ingredient_id: i64,
}

#[derive(Serialize, JsonSchema)]
pub struct GetDishResponse {
    dish: Dish,
    added_ingredients: Vec<AddedIngredient>,
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
            id, creation_date, name, prep_date, total_weight
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
            ingredient_id
        FROM Dish
            JOIN DishIngredient ON Dish.id = DishIngredient.dish_id
            JOIN Ingredient on DishIngredient.ingredient_id = Ingredient.id
        WHERE Dish.id = ?;
        "#,
        id
    )
    .fetch_all(&connection)
    .await?
    .into_iter()
    .collect();

    Ok(ServerResponse::success(GetDishResponse {
        dish,
        added_ingredients,
    })
    .json())
}
