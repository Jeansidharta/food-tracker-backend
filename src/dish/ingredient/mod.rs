use diesel::prelude::*;

#[derive(Deserialize, Serialize)]
pub struct DishIngredient {
    id: u32,
    dish_id: u32,
    ingredient_id: u32,
    creation_date: u32,
    weight: u32,
}

use aide::axum::ApiRouter;
use serde::{Deserialize, Serialize};

use axum::{extract::Path, routing::post, Json};

use crate::{app_error::AppError, establish_connection, models, server::ServerResponse};

#[derive(Deserialize)]
struct AddIngredientToDish {
    ingredient_id: i32,
    weight: i32,
}

async fn post_dish_ingredient(
    Path(dish_id): Path<i32>,
    Json(AddIngredientToDish {
        ingredient_id,
        weight,
    }): Json<AddIngredientToDish>,
) -> Result<Json<ServerResponse<models::DishIngredient>>, AppError> {
    use crate::schema::DishIngredient::dsl::DishIngredient;
    // let data = Dish::database_add_ingredient(dish_id, ingredient_id, weight)?;
    let mut connection = establish_connection();

    let data = diesel::insert_into(DishIngredient)
        .values(models::NewDishIngredient {
            dish_id,
            ingredient_id,
            weight,
        })
        .returning(models::DishIngredient::as_returning())
        .get_result(&mut connection)?;

    Ok(Json::from(ServerResponse {
        message: "Success".to_string(),
        data,
    }))
}

pub fn route() -> ApiRouter {
    ApiRouter::new().route("/", post(post_dish_ingredient))
}
