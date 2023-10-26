use aide::axum::{
    routing::{get, post},
    ApiRouter,
};
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::{establish_connection, models, server::ServerResponse};
use axum::{extract::Path, Json};

use crate::app_error::AppError;

#[derive(Deserialize, JsonSchema)]
struct PostMealDish {
    weight: i32,
    dish_id: i32,
}

#[derive(Deserialize, JsonSchema)]
struct PostMeal {
    pub eat_date: Option<i32>,
    pub duration: Option<i32>,
    pub description: Option<String>,
    pub hunger_level: Option<i32>,
    pub desire_to_eat: Option<i32>,
    pub fullness_afterwards: Option<i32>,
    pub dishes: Vec<PostMealDish>,
}

impl Into<models::NewMeal> for PostMeal {
    fn into(self) -> models::NewMeal {
        models::NewMeal {
            eat_date: self.eat_date,
            duration: self.duration,
            description: self.description,
            hunger_level: self.hunger_level,
            desire_to_eat: self.desire_to_eat,
            fullness_afterwards: self.fullness_afterwards,
        }
    }
}

async fn get_meal(
    Path(id): Path<i32>,
) -> Result<Json<ServerResponse<Option<models::Meal>>>, AppError> {
    use crate::schema::Meal::dsl::Meal;

    let mut connection = establish_connection();

    let data = Meal
        .find(id)
        .select(models::Meal::as_select())
        .get_results(&mut connection)?
        .pop();

    Ok(Json::from(ServerResponse {
        message: "Success".to_string(),
        data,
    }))
}

async fn list_meal() -> Result<Json<ServerResponse<Vec<models::Meal>>>, AppError> {
    use crate::schema::Meal::dsl::*;

    let mut connection = establish_connection();

    let data = Meal
        .select(models::Meal::as_select())
        .get_results(&mut connection)?;

    Ok(Json::from(ServerResponse {
        message: "Success".to_string(),
        data,
    }))
}

async fn post_meal(
    Json(post_meal): Json<PostMeal>,
) -> Result<Json<ServerResponse<(models::Meal, Vec<models::MealDish>)>>, AppError> {
    use crate::schema::Meal::dsl::Meal;
    use crate::schema::MealDish::dsl::MealDish;

    let mut connection = establish_connection();

    let meal = diesel::insert_into(Meal)
        .values(models::NewMeal {
            eat_date: post_meal.eat_date,
            duration: post_meal.duration,
            description: post_meal.description,
            hunger_level: post_meal.hunger_level,
            desire_to_eat: post_meal.desire_to_eat,
            fullness_afterwards: post_meal.fullness_afterwards,
        })
        .returning(models::Meal::as_returning())
        .get_result(&mut connection)?;

    let meal_id = meal.id;
    let dishes = post_meal
        .dishes
        .into_iter()
        .map(|dish| {
            diesel::insert_into(MealDish)
                .values(models::NewMealDish {
                    meal_id,
                    dish_id: dish.dish_id,
                    weight: dish.weight,
                })
                .returning(models::MealDish::as_returning())
                .get_result(&mut connection)
        })
        .collect::<diesel::result::QueryResult<Vec<models::MealDish>>>()?;

    Ok(Json::from(ServerResponse {
        message: "Success".to_string(),
        data: (meal, dishes),
    }))
}

pub fn route() -> ApiRouter {
    ApiRouter::new()
        .api_route("/", post(post_meal).get(list_meal))
        .api_route("/:id", get(get_meal))
}
