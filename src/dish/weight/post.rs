use axum::{
    extract::{Path, State},
    Json,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::QueryBuilder;
use thiserror::Error;

use crate::{
    get_missing_items,
    models::{Dish, DishIngredient},
    server::{ServerResponse, ServerResponseResult},
    state::AppState,
};

#[derive(Deserialize, JsonSchema, Debug)]
struct PostDishIngredient {
    weight: i64,
    ingredient_id: i64,
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct PostTotalWeight {
    total_weight: i64,
}

#[derive(Serialize, JsonSchema)]
pub struct TotalWeightResponse {
    total_weight: i64,
}

#[derive(Deserialize, JsonSchema)]
pub struct DishId {
    dish_id: i64,
}

#[derive(Error, Debug)]
enum PostDishError {
    #[error("Dish with id {0} doesn't exist")]
    UnknownDishId(i64),
    #[error("Weight {0} is invalid. It must be larger than 0")]
    InvalidWeight(i64),
}
pub async fn post_weight(
    State(AppState { connection }): State<AppState>,
    Path(DishId { dish_id: id }): Path<DishId>,
    Json(PostTotalWeight { total_weight }): Json<PostTotalWeight>,
) -> ServerResponseResult<TotalWeightResponse> {
    if total_weight < 0 {
        return Err(PostDishError::InvalidWeight(total_weight))?;
    }

    let total_weight = sqlx::query_scalar!(
        r#"
        UPDATE Dish
        SET total_weight = ?
        WHERE Dish.id = ?
        RETURNING total_weight;"#,
        total_weight,
        id
    )
    .fetch_optional(&connection)
    .await?
    .ok_or(PostDishError::UnknownDishId(id))?;

    Ok(ServerResponse::success(TotalWeightResponse { total_weight }).json())
}
