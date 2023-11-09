use aide::axum::{
    routing::{get, post},
    ApiRouter,
};

use crate::state::AppState;

mod get;
mod list;
mod post;

pub fn route(state: AppState) -> ApiRouter {
    ApiRouter::new()
        .api_route("/", post(post::post_ingredient).get(list::list_ingredients))
        .api_route("/:id", get(get::get_ingredient))
        .with_state(state)
}
