use aide::axum::{
    routing::{get, post},
    ApiRouter,
};
use diesel::prelude::*;
mod ingredient;

use anyhow::Result;
use axum::{extract::Path, Json};
use schemars::JsonSchema;
use serde::Deserialize;
use thiserror::Error;

use crate::{app_error::AppError, establish_connection, models, server::ServerResponse};

use ingredient::route as route_dish_ingredient;

#[derive(Deserialize, JsonSchema)]
struct PostDishIngredient {
    weight: i32,
    ingredient_id: i32,
}

#[derive(Deserialize, JsonSchema)]
struct PostDish {
    name: String,
    prep_date: Option<i32>,
    dish_ingredients: Option<Vec<PostDishIngredient>>,
}

async fn get_dish(
    Path(id): Path<i32>,
) -> Result<Json<ServerResponse<Option<models::Dish>>>, AppError> {
    use crate::schema::Dish::dsl::Dish;
    let mut connection = establish_connection();

    let data = Dish
        .find(id)
        .select(models::Dish::as_select())
        .get_results(&mut connection)?
        .pop();

    Ok(Json::from(ServerResponse {
        message: "Success".to_string(),
        data,
    }))
}

async fn list_dish() -> Result<Json<ServerResponse<Vec<models::Dish>>>, AppError> {
    use crate::schema::Dish::dsl::Dish;
    let mut connection = establish_connection();
    let data = Dish
        .select(models::Dish::as_select())
        .get_results(&mut connection)?;

    Ok(Json::from(ServerResponse {
        message: "Success".to_string(),
        data,
    }))
}

#[derive(Error, Debug)]
enum PostDishError {
    #[error("The following ingredients don't exits: {0:?}")]
    UnknownIngredientId(Vec<i32>),
}

async fn post_dish(
    Json(PostDish {
        name,
        prep_date,
        dish_ingredients,
    }): Json<PostDish>,
) -> Result<Json<ServerResponse<(models::Dish, Vec<models::DishIngredient>)>>, AppError> {
    use crate::schema::Dish::dsl::Dish;
    use crate::schema::DishIngredient::dsl::DishIngredient;
    use crate::schema::Ingredient::dsl::Ingredient;

    let mut connection = establish_connection();
    let dish_ingredients = dish_ingredients.unwrap_or_default();
    let unknown_ingredients: Vec<i32> = dish_ingredients
        .iter()
        .map(
            |PostDishIngredient {
                 ingredient_id: id, ..
             }| {
                (
                    Ingredient
                        .find(id)
                        .select(models::Ingredient::as_select())
                        .first(&mut connection),
                    id,
                )
            },
        )
        .filter(|(res, _)| res.is_err())
        .map(|(_, id)| *id)
        .collect();

    if !unknown_ingredients.is_empty() {
        return Err(PostDishError::UnknownIngredientId(unknown_ingredients))?;
    }

    let data = connection.transaction(
        move |connection| -> std::result::Result<
            (models::Dish, Vec<models::DishIngredient>),
            diesel::result::Error,
        > {
            let new_dish = diesel::insert_into(Dish)
                .values(models::NewDish {
                    name: Some(name),
                    prep_date,
                })
                .returning(models::Dish::as_returning())
                .get_result(connection)?;

            let new_dish_ingredients = dish_ingredients
                .into_iter()
                .map(
                    |PostDishIngredient {
                         weight,
                         ingredient_id,
                     }| {
                        diesel::insert_into(DishIngredient)
                            .values(models::NewDishIngredient {
                                dish_id: new_dish.id,
                                ingredient_id,
                                weight,
                            })
                            .returning(models::DishIngredient::as_returning())
                            .get_result(connection)
                    },
                )
                .collect::<Result<Vec<models::DishIngredient>, diesel::result::Error>>()?;
            Ok((new_dish, new_dish_ingredients))
        },
    )?;

    Ok(Json::from(ServerResponse {
        message: "Success".to_string(),
        data,
    }))
}

pub fn route() -> ApiRouter {
    ApiRouter::new()
        .api_route("/", post(post_dish).get(list_dish))
        .api_route("/:id", get(get_dish))
        .nest_api_service("/:id/ingredient", route_dish_ingredient())
}
