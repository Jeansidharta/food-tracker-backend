use axum::extract::State;
use schemars::JsonSchema;
use serde::Serialize;

use crate::{
    server::{ServerResponse, ServerResponseResult},
    state::AppState,
};

pub struct ListDishQuery {
    id: i64,
    name: Option<String>,
    prep_date: Option<i64>,
    creation_date: i64,
    ingredients_count: Option<i64>,
}

#[derive(JsonSchema, Serialize)]
pub struct ListDishResponse {
    id: i64,
    name: Option<String>,
    prep_date: Option<i64>,
    creation_date: i64,
    ingredients_count: i64,
}

pub async fn list_dish(
    State(AppState { connection }): State<AppState>,
) -> ServerResponseResult<Vec<ListDishResponse>> {
    let dishes = sqlx::query_as_unchecked!(
        ListDishQuery,
        r#"
        SELECT 
            (SELECT COUNT(DishIngredient.dish_id) FROM DishIngredient WHERE DishIngredient.dish_id = Dish.id) AS ingredients_count,
            id,
            name,
            creation_date,
            prep_date
        FROM Dish
        ORDER BY prep_date DESC"#
    )
    .fetch_all(&connection)
    .await?
    .into_iter()
    .map(|i| ListDishResponse {
        ingredients_count: i.ingredients_count.unwrap_or(0),
        id: i.id,
        name: i.name,
        prep_date: i.prep_date,
        creation_date: i.creation_date,
    })
    .collect();

    Ok(ServerResponse::success(dishes).json())
}
