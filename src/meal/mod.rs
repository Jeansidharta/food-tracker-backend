use aide::axum::{routing::post, ApiRouter};

use crate::state::AppState;

mod _id;
mod component;
mod description;
mod list;
mod post;

pub fn route(state: AppState) -> ApiRouter {
    ApiRouter::new()
        .api_route("/", post(post::post_meal).get(list::list_meal))
        .nest_api_service("/description", description::route(state.clone()))
        .nest_api_service("/component", component::route(state.clone()))
        .nest_api_service("/:meal_id", _id::route(state.clone()))
        .with_state(state)
}
