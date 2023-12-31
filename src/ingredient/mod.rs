use aide::axum::{routing::post, ApiRouter};

use crate::state::AppState;

mod _id;
mod list;
mod post;

pub fn route(state: AppState) -> ApiRouter {
    ApiRouter::new()
        .api_route("/", post(post::post_ingredient).get(list::list_ingredients))
        .nest_api_service("/:ingredient_id", _id::route(state.clone()))
        .with_state(state)
}
