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
pub struct DishId {
    dish_id: i64,
}

#[derive(Deserialize, JsonSchema)]
pub struct PostIngredientBody {
    weight: i64,
    ingredient_id: i64,
}

#[derive(Serialize, JsonSchema)]
pub struct PostIngredientResult {
    weight: i64,
    ingredient_id: i64,
    dish_id: i64,
    creation_date: i64,
}

pub async fn post_ingredient(
    State(AppState { connection }): State<AppState>,
    Path(DishId { dish_id }): Path<DishId>,
    Json(PostIngredientBody {
        weight,
        ingredient_id,
    }): Json<PostIngredientBody>,
) -> ServerResponseResult<PostIngredientResult> {
    let data = sqlx::query_as!(
        PostIngredientResult,
        r#"
        INSERT INTO
            DishIngredient (dish_id, ingredient_id, weight)
            VALUES (?, ?, ?)
        RETURNING dish_id, ingredient_id, weight, creation_date;
        "#,
        dish_id,
        ingredient_id,
        weight
    )
    .fetch_one(&connection)
    .await?;

    Ok(ServerResponse::success(data).json())
}
