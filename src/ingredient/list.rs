use axum::extract::State;

use crate::{
    models::Ingredient,
    server::{ServerResponse, ServerResponseResult},
    state::AppState,
};

pub async fn list_ingredients(
    State(AppState { connection }): State<AppState>,
) -> ServerResponseResult<Vec<Ingredient>> {
    let ingredients = sqlx::query_as!(Ingredient, "select * from Ingredient;")
        .fetch_all(&connection)
        .await?;

    Ok(ServerResponse::success(ingredients).json())
}
