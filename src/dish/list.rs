use axum::extract::{Query, State};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::{
    server::{ServerResponse, ServerResponseResult},
    state::AppState,
};

#[derive(FromRow, JsonSchema, Serialize)]
pub struct ListDishResponse {
    id: i64,
    name: Option<String>,
    prep_date: Option<i64>,
    creation_date: i64,
    is_finished: i64,
}

#[derive(JsonSchema, Deserialize)]
pub struct ListDishQueryParams {
    is_finished: Option<bool>,
}

pub async fn list_dish(
    State(AppState { connection }): State<AppState>,
    Query(query_params): Query<ListDishQueryParams>,
) -> ServerResponseResult<Vec<ListDishResponse>> {
    let mut queries = sqlx::QueryBuilder::new(
        r#"
        SELECT 
            id,
            name,
            creation_date,
            prep_date,
            is_finished
        FROM Dish
        "#,
    );

    if let Some(is_finished) = query_params.is_finished {
        queries
            .push("WHERE is_finished = ")
            .push(is_finished)
            .push("\n");
    }

    let dishes = queries
        .push("ORDER BY prep_date DESC")
        .build_query_as::<ListDishResponse>()
        .fetch_all(&connection)
        .await?;

    Ok(ServerResponse::success(dishes).json())
}
