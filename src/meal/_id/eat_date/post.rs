use axum::{
    extract::{Path, State},
    Json,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    models::Meal,
    server::{ServerResponse, ServerResponseResult},
    state::AppState,
};

#[derive(Deserialize, JsonSchema)]
pub struct MealId {
    meal_id: i64,
}

#[derive(Deserialize, JsonSchema)]
pub struct PostEatDateBody {
    pub eat_date: i64,
}

#[derive(Serialize, JsonSchema)]
pub struct PostEatDateResult {
    meal: Meal,
}

pub async fn post_eat_date(
    State(AppState { connection }): State<AppState>,
    Path(MealId { meal_id: id }): Path<MealId>,
    Json(post_eat_date): Json<PostEatDateBody>,
) -> ServerResponseResult<PostEatDateResult> {
    let meal = sqlx::query_as!(
        Meal,
        r#"UPDATE Meal SET
            eat_date = ?
        WHERE id = ?
        RETURNING id as 'id!', creation_date, eat_date, duration, description;"#,
        post_eat_date.eat_date,
        id
    )
    .fetch_one(&connection)
    .await?;

    Ok(ServerResponse::success(PostEatDateResult { meal }).json())
}
