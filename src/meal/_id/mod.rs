use aide::axum::{routing::get, ApiRouter};

use crate::state::AppState;

mod delete;
mod dish;
mod eat_date;
mod get;
mod ingredient;
mod post;

pub fn route(state: AppState) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            get(get::get_meal)
                .delete(delete::delete_meal)
                .post(post::post_meal),
        )
        .nest_api_service("/dish", dish::route(state.clone()))
        .nest_api_service("/ingredient", ingredient::route(state.clone()))
        .nest_api_service("/eat_date", eat_date::route(state.clone()))
        .with_state(state)
}
