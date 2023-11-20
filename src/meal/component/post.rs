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
enum ComponentType {
    Dish,
    Ingredient,
}

#[derive(Deserialize, JsonSchema)]
pub struct PostComponentBody {
    weight: i64,
    component_id: i64,
    component_type: ComponentType,
}

#[derive(Deserialize, JsonSchema)]
pub struct MealId {
    meal_id: i64,
}

#[derive(Serialize, JsonSchema)]
pub struct PostComponentResult {
    weight: i64,
    meal_id: i64,
    component_id: i64,
    creation_date: i64,
}

pub async fn post_component(
    State(AppState { connection }): State<AppState>,
    Path(MealId { meal_id }): Path<MealId>,
    Json(PostComponentBody {
        weight,
        component_id,
        component_type,
    }): Json<PostComponentBody>,
) -> ServerResponseResult<PostComponentResult> {
    let data = match component_type {
        ComponentType::Dish => {
            sqlx::query_as!(
                PostComponentResult,
                r#"
                INSERT INTO
                    MealDish (dish_id, meal_id, weight)
                VALUES (?, ?, ?)
                RETURNING dish_id as component_id, meal_id, weight, creation_date;
                "#,
                component_id,
                meal_id,
                weight
            )
            .fetch_one(&connection)
            .await?
        }
        ComponentType::Ingredient => {
            sqlx::query_as!(
                PostComponentResult,
                r#"
                INSERT INTO
                    MealIngredient (ingredient_id, meal_id, weight)
                VALUES (?, ?, ?)
                RETURNING ingredient_id as component_id, meal_id, weight, creation_date;
                "#,
                component_id,
                meal_id,
                weight
            )
            .fetch_one(&connection)
            .await?
        }
    };

    Ok(ServerResponse::success(data).json())
}
