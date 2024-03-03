use aide::transform::TransformOperation;
use axum::{extract::State, http::StatusCode, Json};
use schemars::JsonSchema;
use serde::Deserialize;

use crate::{
    app_error::InternalServerError,
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

    Ok(ServerResponse::success_code(data, StatusCode::CREATED).json())
}

pub fn post_ingredient_docs(op: TransformOperation) -> TransformOperation {
    op.response::<201, Json<ServerResponse<Ingredient>>>()
        .response::<500, Json<InternalServerError>>()
}
