use axum::extract::{Path, State};
use schemars::JsonSchema;
use serde::Deserialize;

use crate::{
    server::{ServerResponse, ServerResponseResult},
    state::AppState,
};

#[derive(Deserialize, JsonSchema)]
pub struct DishId {
    dish_id: i64,
}

#[derive(Deserialize, JsonSchema)]
pub struct IngredientId {
    ingredient_id: i64,
}

pub async fn delete_ingredient(
    State(AppState { connection }): State<AppState>,
    Path(DishId { dish_id }): Path<DishId>,
    Path(IngredientId { ingredient_id }): Path<IngredientId>,
) -> ServerResponseResult<bool> {
    sqlx::query!(
        r#"
        DELETE FROM DishIngredient WHERE dish_id = ? AND ingredient_id = ?"#,
        dish_id,
        ingredient_id,
    )
    .execute(&connection)
    .await?;

    Ok(ServerResponse::success(true).json())
}
