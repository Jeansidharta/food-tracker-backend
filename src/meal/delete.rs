use axum::extract::{Path, State};
use schemars::JsonSchema;
use serde::Deserialize;

use crate::{
    server::{ServerResponse, ServerResponseResult}, state::AppState
};

#[derive(Deserialize, JsonSchema)]
pub struct MealId {
    meal_id: i64,
}

pub async fn delete_meal(
    State(AppState { connection }): State<AppState>,
    Path(MealId { meal_id }): Path<MealId>,
) -> ServerResponseResult<bool> {
    sqlx::query!(
        r#"
        DELETE FROM MealDish WHERE meal_id = ?;
        DELETE FROM Meal WHERE id = ?"#,
        meal_id,
        meal_id,
    )
    .execute(&connection)
    .await?;

    Ok(ServerResponse::success(true).json())
}
