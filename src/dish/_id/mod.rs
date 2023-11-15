use aide::axum::{routing::get, ApiRouter};

use crate::state::AppState;

mod delete;
mod delete_warning;
mod get;
mod ingredient;
mod post;
mod weight;

pub fn route(state: AppState) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            get(get::get_dish)
                .delete(delete::delete_dish)
                .post(post::post_edit_dish),
        )
        .api_route("/delete-warning", get(delete_warning::get_delete_warning))
        .nest_api_service("/weight", weight::route(state.clone()))
        .nest_api_service("/ingredient", ingredient::route(state.clone()))
        .with_state(state)
}
