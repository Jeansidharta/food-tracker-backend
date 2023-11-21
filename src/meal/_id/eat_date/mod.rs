use aide::axum::{routing::post, ApiRouter};

use crate::state::AppState;

mod post;

pub fn route(state: AppState) -> ApiRouter {
    ApiRouter::new()
        .api_route("/", post(post::post_eat_date))
        .with_state(state)
}
