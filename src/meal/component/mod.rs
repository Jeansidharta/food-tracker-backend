use aide::axum::{routing::get, ApiRouter};

use crate::state::AppState;

mod get;
mod post;

pub fn route(state: AppState) -> ApiRouter {
    ApiRouter::new()
        .api_route("/", get(get::get_component).post(post::post_component))
        .with_state(state)
}
