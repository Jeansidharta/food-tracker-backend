use aide::transform::TransformOperation;
use axum::{extract::State, http::StatusCode, Json};
use schemars::JsonSchema;
use serde::Serialize;

use crate::{
    app_error::InternalServerError,
    models::Ingredient,
    server::{ServerResponse, ServerResponseResult},
    state::AppState,
};

#[derive(Serialize, JsonSchema)]
pub struct GetIngredientResponse {
    pub ingredients: Vec<Ingredient>,
}

pub async fn list_ingredients(
    State(AppState { connection }): State<AppState>,
) -> ServerResponseResult<GetIngredientResponse> {
    let ingredients = sqlx::query_as!(Ingredient, "select * from Ingredient;")
        .fetch_all(&connection)
        .await?;

    Ok(
        ServerResponse::success_code(GetIngredientResponse { ingredients }, StatusCode::CREATED)
            .json(),
    )
}

pub fn list_ingredients_docs(op: TransformOperation) -> TransformOperation {
    op.response::<201, Json<ServerResponse<GetIngredientResponse>>>()
        .response::<500, Json<InternalServerError>>()
}
