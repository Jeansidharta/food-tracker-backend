use aide::axum::{routing::post_with, ApiRouter};

use crate::state::AppState;

use self::post::{post_ingredient_properties, post_ingredient_properties_docs};

mod post;

pub fn route(state: AppState) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            post_with(post_ingredient_properties, post_ingredient_properties_docs),
        )
        .with_state(state)
}
