use aide::axum::{
    routing::{delete, post},
    ApiRouter,
};

use crate::state::AppState;

mod delete;
mod post;

pub fn route(state: AppState) -> ApiRouter {
    ApiRouter::new()
        .api_route("/", post(post::post_dish))
        .api_route("/:dish_id", delete(delete::delete_dish))
        .with_state(state)
}
