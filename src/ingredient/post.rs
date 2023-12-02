use axum::{extract::State, Json};
use schemars::JsonSchema;
use serde::Deserialize;

use crate::{
    models::Ingredient,
    server::{ServerResponse, ServerResponseResult},
    state::AppState,
};

#[derive(Deserialize, JsonSchema)]
pub struct PostIngredientBody {
    pub name: String,
}

pub async fn post_ingredient(
    State(AppState { connection }): State<AppState>,
    Json(PostIngredientBody { name }): Json<PostIngredientBody>,
) -> ServerResponseResult<Ingredient> {
    let data = sqlx::query_as!(
        Ingredient,
        "INSERT INTO Ingredient (name) VALUES (?) RETURNING *;",
        name
    )
    .fetch_one(&connection)
    .await?;

    Ok(ServerResponse::success(data).json())
}
