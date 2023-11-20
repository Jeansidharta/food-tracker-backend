use aide::axum::{
    routing::{get, post},
    ApiRouter,
};

use crate::state::AppState;

mod component;
mod delete;
mod description;
mod dish;
mod get;
mod ingredient;
mod list;
mod post;

pub fn route(state: AppState) -> ApiRouter {
    ApiRouter::new()
        .api_route("/", post(post::post_meal).get(list::list_meal))
        .api_route("/:meal_id", get(get::get_meal).delete(delete::delete_meal))
        .nest_api_service("/description", description::route(state.clone()))
        .nest_api_service("/component", component::route(state.clone()))
        .nest_api_service("/:meal_id/dish", dish::route(state.clone()))
        .nest_api_service("/:meal_id/ingredient", ingredient::route(state.clone()))
        .with_state(state)
}
