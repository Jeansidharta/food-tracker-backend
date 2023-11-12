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
    let current_weight = sqlx::query_scalar!(
        r#"
        SELECT weight
        FROM DishIngredient 
        WHERE dish_id = ? AND ingredient_id = ?"#,
        dish_id,
        ingredient_id
    )
    .fetch_optional(&connection)
    .await?;

    let data = if let Some(current_weight) = current_weight {
        let new_weight = weight + current_weight;
        sqlx::query_as!(
            PostIngredientResult,
            r#"
            UPDATE DishIngredient
            SET weight = ?
            WHERE dish_id = ? AND ingredient_id = ?
            RETURNING dish_id, ingredient_id, weight, creation_date;
            "#,
            new_weight,
            dish_id,
            ingredient_id
        )
        .fetch_one(&connection)
        .await?
    } else {
        sqlx::query_as!(
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
        .await?
    };

    Ok(ServerResponse::success(data).json())
}
