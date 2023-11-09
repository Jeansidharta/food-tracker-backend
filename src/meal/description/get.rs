use axum::extract::State;

use crate::{
    server::{ServerResponse, ServerResponseResult},
    state::AppState,
};

pub async fn get_descriptions(
    State(AppState { connection }): State<AppState>,
) -> ServerResponseResult<Vec<String>> {
    let descriptions = sqlx::query_scalar!("SELECT description FROM UsualMealDescriptions;")
        .fetch_all(&connection)
        .await?;

    Ok(ServerResponse::success(descriptions).json())
}
