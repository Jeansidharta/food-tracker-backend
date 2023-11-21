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
pub struct MealId {
    meal_id: i64,
}

pub async fn delete_dish(
    State(AppState { connection }): State<AppState>,
    Path(DishId { dish_id }): Path<DishId>,
    Path(MealId { meal_id }): Path<MealId>,
) -> ServerResponseResult<bool> {
    sqlx::query!(
        r#"
        DELETE FROM MealDish WHERE dish_id = ? AND meal_id = ?"#,
        dish_id,
        meal_id,
    )
    .execute(&connection)
    .await?;

    Ok(ServerResponse::success(true).json())
}
