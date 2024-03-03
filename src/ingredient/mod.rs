use aide::axum::{routing::post_with, ApiRouter};

use crate::state::AppState;

use self::{get::list_ingredients_docs, post::post_ingredient_docs};

mod _id;
mod get;
mod post;

pub fn route(state: AppState) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            post_with(post::post_ingredient, post_ingredient_docs)
                .get_with(get::list_ingredients, list_ingredients_docs),
        )
        .nest_api_service("/:ingredient_id", _id::route(state.clone()))
        .with_state(state)
}
