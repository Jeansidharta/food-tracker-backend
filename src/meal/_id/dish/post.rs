use axum::{
    extract::{Path, State},
    Json,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    server::{ServerResponse, ServerResponseResult},
    state::AppState,
};

#[derive(Deserialize, JsonSchema)]
pub struct PostDishBody {
    weight: i64,
    dish_id: i64,
}

#[derive(Deserialize, JsonSchema)]
pub struct MealId {
    meal_id: i64,
}

#[derive(Serialize, JsonSchema)]
pub struct PostDishResult {
    weight: i64,
    meal_id: i64,
    dish_id: i64,
    creation_date: i64,
}

pub async fn post_dish(
    State(AppState { connection }): State<AppState>,
    Path(MealId { meal_id }): Path<MealId>,
    Json(PostDishBody { weight, dish_id }): Json<PostDishBody>,
) -> ServerResponseResult<PostDishResult> {
    let data = sqlx::query_as!(
        PostDishResult,
        r#"
        INSERT INTO
            MealDish (dish_id, meal_id, weight)
        VALUES (?, ?, ?)
        RETURNING dish_id, meal_id, weight, creation_date;
        "#,
        dish_id,
        meal_id,
        weight
    )
    .fetch_one(&connection)
    .await?;

    Ok(ServerResponse::success(data).json())
}
