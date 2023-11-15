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

pub async fn delete_dish(
    State(AppState { connection }): State<AppState>,
    Path(DishId { dish_id }): Path<DishId>,
) -> ServerResponseResult<bool> {
    sqlx::query!(
        r#"
        DELETE FROM MealDish WHERE dish_id = ?;
        DELETE FROM Dish WHERE id = ?"#,
        dish_id,
        dish_id,
    )
    .execute(&connection)
    .await?;

    Ok(ServerResponse::success(true).json())
}
