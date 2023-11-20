use axum::extract::{Path, State};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    models::Meal,
    server::{ServerResponse, ServerResponseResult},
    state::AppState,
};

#[derive(Serialize, JsonSchema)]
pub struct MealDish {
    weight: i64,
    dish_name: Option<String>,
    dish_id: i64,
}

#[derive(Serialize, JsonSchema)]
pub struct GetMealResponse {
    meal: Meal,
    dishes: Vec<MealDish>,
}

#[derive(Error, Debug)]
enum GetMeal {
    #[error("Could not find meal with id \"{0}\"")]
    MealNotFound(i64),
}

#[derive(Deserialize, JsonSchema)]
pub struct MealId {
    meal_id: i64,
}

pub async fn get_meal(
    State(AppState { connection }): State<AppState>,
    Path(MealId { meal_id: id }): Path<MealId>,
) -> ServerResponseResult<GetMealResponse> {
    let meal = sqlx::query_as!(
        Meal,
        r#"
        SELECT
            id,
            creation_date,
            duration,
            description,
            eat_date
        FROM Meal
        WHERE id = ?;"#,
        id
    )
    .fetch_optional(&connection)
    .await?
    .ok_or(GetMeal::MealNotFound(id))?;

    let dishes = sqlx::query_as!(
        MealDish,
        r#"
        SELECT 
            MealDish.weight,
            Dish.name as dish_name,
            Dish.id as dish_id
        FROM Meal
            JOIN MealDish ON Meal.id = MealDish.meal_id
            JOIN Dish ON MealDish.dish_id = Dish.id
        WHERE Meal.id = ?;
        "#,
        id
    )
    .fetch_all(&connection)
    .await?;

    Ok(ServerResponse::success(GetMealResponse { meal, dishes }).json())
}
