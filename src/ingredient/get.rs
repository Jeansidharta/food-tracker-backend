use axum::extract::{Path, State};
use thiserror::Error;

use crate::{
    models::Ingredient,
    server::{ServerResponse, ServerResponseResult},
    state::AppState,
    PathId,
};

#[derive(Error, Debug)]
enum GetIngredient {
    #[error("Could not find ingredient with id \"{0}\"")]
    IngredientNotFound(i64),
}

pub async fn get_ingredient(
    State(AppState { connection }): State<AppState>,
    Path(PathId { id }): Path<PathId>,
) -> ServerResponseResult<Ingredient> {
    let ingredient = sqlx::query_as!(
        Ingredient,
        "SELECT name, id, creation_date FROM Ingredient WHERE id=?;",
        id
    )
    .fetch_optional(&connection)
    .await?
    .ok_or(GetIngredient::IngredientNotFound(id))?;

    Ok(ServerResponse::success(ingredient).json())
}
