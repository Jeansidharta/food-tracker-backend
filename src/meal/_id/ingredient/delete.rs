use axum::extract::{Path, State};
use schemars::JsonSchema;
use serde::Deserialize;

use crate::{
    server::{ServerResponse, ServerResponseResult},
    state::AppState,
};

#[derive(Deserialize, JsonSchema)]
pub struct IngredientId {
    ingredient_id: i64,
}

#[derive(Deserialize, JsonSchema)]
pub struct MealId {
    meal_id: i64,
}

pub async fn delete_ingredient(
    State(AppState { connection }): State<AppState>,
    Path(IngredientId { ingredient_id }): Path<IngredientId>,
    Path(MealId { meal_id }): Path<MealId>,
) -> ServerResponseResult<bool> {
    sqlx::query!(
        r#"
        DELETE FROM MealIngredient WHERE ingredient_id = ? AND meal_id = ?"#,
        ingredient_id,
        meal_id,
    )
    .execute(&connection)
    .await?;

    Ok(ServerResponse::success(true).json())
}
