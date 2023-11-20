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
pub struct PostIngredientBody {
    weight: i64,
    ingredient_id: i64,
}

#[derive(Deserialize, JsonSchema)]
pub struct MealId {
    meal_id: i64,
}

#[derive(Serialize, JsonSchema)]
pub struct PostIngredientResult {
    weight: i64,
    meal_id: i64,
    ingredient_id: i64,
    creation_date: i64,
}

pub async fn post_ingredient(
    State(AppState { connection }): State<AppState>,
    Path(MealId { meal_id }): Path<MealId>,
    Json(PostIngredientBody { weight, ingredient_id }): Json<PostIngredientBody>,
) -> ServerResponseResult<PostIngredientResult> {
    let data = sqlx::query_as!(
        PostIngredientResult,
        r#"
        INSERT INTO
            MealIngredient (ingredient_id, meal_id, weight)
        VALUES (?, ?, ?)
        RETURNING ingredient_id, meal_id, weight, creation_date;
        "#,
        ingredient_id,
        meal_id,
        weight
    )
    .fetch_one(&connection)
    .await?;

    Ok(ServerResponse::success(data).json())
}
