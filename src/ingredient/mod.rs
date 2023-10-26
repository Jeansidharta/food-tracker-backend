use crate::models::NewIngredient;
use crate::schema::Ingredient::dsl::Ingredient;
use crate::{app_error::AppError, establish_connection, models, server::ServerResponse};
use aide::axum::{
    routing::{get, post},
    ApiRouter,
};
use axum::{extract::Path, Json};
use diesel::prelude::*;

async fn post_ingredient(
    Json(new_ingredient): Json<NewIngredient>,
) -> Result<Json<ServerResponse<models::Ingredient>>, AppError> {
    let mut connection = establish_connection();

    let data = diesel::insert_into(Ingredient)
        .values(new_ingredient)
        .returning(models::Ingredient::as_returning())
        .get_result(&mut connection)?;

    Ok(Json::from(ServerResponse {
        message: "Success".to_string(),
        data,
    }))
}

async fn list_ingredients() -> Result<Json<ServerResponse<Vec<models::Ingredient>>>, AppError> {
    use crate::schema::Ingredient::dsl::*;
    let mut connection = establish_connection();

    let ingredients = Ingredient
        .select(crate::models::Ingredient::as_select())
        .load(&mut connection)?;

    Ok(Json::from(ServerResponse {
        message: "Success".to_string(),
        data: ingredients,
    }))
}

async fn get_ingredient(
    Path(id): Path<i32>,
) -> Result<Json<ServerResponse<Option<models::Ingredient>>>, AppError> {
    let mut connection = establish_connection();
    let data = Ingredient
        .find(id)
        .select(models::Ingredient::as_select())
        .get_results(&mut connection)?
        .pop();

    Ok(Json::from(ServerResponse {
        message: "Success".to_string(),
        data,
    }))
}

pub fn route() -> ApiRouter {
    ApiRouter::new()
        .api_route("/", post(post_ingredient).get(list_ingredients))
        .api_route("/:id", get(get_ingredient))
}
