use axum::extract::{Path, State};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    server::{ServerResponse, ServerResponseResult},
    state::AppState,
};

#[derive(Error, Debug)]
enum GetIngredient {
    #[error("Could not find ingredient with id \"{0}\"")]
    IngredientNotFound(i64),
}

#[derive(JsonSchema, Deserialize)]
pub struct IngredientId {
    pub ingredient_id: i64,
}

#[derive(JsonSchema, Serialize)]
pub struct GetIngredientResult {
    id: i64,
    name: Option<String>,
    creation_date: i64,
    kcal_100g: Option<i64>,
    product_code: Option<String>,
}

pub async fn get_ingredient(
    State(AppState { connection }): State<AppState>,
    Path(IngredientId { ingredient_id }): Path<IngredientId>,
) -> ServerResponseResult<GetIngredientResult> {
    let ingredient = sqlx::query_as!(
        GetIngredientResult,
        r#"
        SELECT
            name,
            id,
            creation_date,
            product_code,
            kcal_100g
        FROM Ingredient
        LEFT JOIN IngredientProperties ON Ingredient.id = IngredientProperties.ingredient_id
        WHERE id=?"#,
        ingredient_id
    )
    .fetch_optional(&connection)
    .await?
    .ok_or(GetIngredient::IngredientNotFound(ingredient_id))?;

    Ok(ServerResponse::success(ingredient).json())
}
