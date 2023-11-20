use aide::axum::{
    routing::{delete, post},
    ApiRouter,
};

use crate::state::AppState;

mod delete;
mod post;

pub fn route(state: AppState) -> ApiRouter {
    ApiRouter::new()
        .api_route("/", post(post::post_ingredient))
        .api_route("/:ingredient_id", delete(delete::delete_ingredient))
        .with_state(state)
}
