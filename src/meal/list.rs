use axum::extract::State;

use crate::{
    models::Meal,
    server::{ServerResponse, ServerResponseResult},
    state::AppState,
};

pub async fn list_meal(
    State(AppState { connection }): State<AppState>,
) -> ServerResponseResult<Vec<Meal>> {
    let meals = sqlx::query_as!(
        Meal,
        r#"
        SELECT
            id,
            creation_date,
            duration,
            desire_to_eat,
            description,
            eat_date,
            fullness_afterwards,
            hunger_level
        FROM Meal
        ORDER BY eat_date DESC;"#
    )
    .fetch_all(&connection)
    .await?;

    Ok(ServerResponse::success(meals).json())
}
