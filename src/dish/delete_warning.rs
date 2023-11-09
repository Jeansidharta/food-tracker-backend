use axum::extract::{Path, State};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    server::{ServerResponse, ServerResponseResult},
    state::AppState,
};

#[derive(Serialize, JsonSchema)]
pub struct GetDeleteWarningResult {
    meal_description: Option<String>,
    meal_eat_date: Option<i64>,
    weight: i64,
}

#[derive(Deserialize, JsonSchema)]
pub struct DishId {
    dish_id: i64,
}

pub async fn get_delete_warning(
    State(AppState { connection }): State<AppState>,
    Path(DishId { dish_id }): Path<DishId>,
) -> ServerResponseResult<Vec<GetDeleteWarningResult>> {
    let results = sqlx::query_as!(
        GetDeleteWarningResult,
        r#"
        SELECT
            Meal.description as meal_description,
            Meal.eat_date as meal_eat_date,
            MealDish.weight as weight
        FROM Dish
        JOIN MealDish ON MealDish.dish_id = Dish.id
        JOIN Meal ON MealDish.meal_id = Meal.id
        WHERE Dish.id = ?;"#,
        dish_id
    )
    .fetch_all(&connection)
    .await?;

    Ok(ServerResponse::success(results).json())
}
