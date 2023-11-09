use axum::{extract::State, Json};

use crate::{
    models::{Ingredient, NewIngredient},
    server::{ServerResponse, ServerResponseResult},
    state::AppState,
};

pub async fn post_ingredient(
    State(AppState { connection }): State<AppState>,
    Json(NewIngredient { name }): Json<NewIngredient>,
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
