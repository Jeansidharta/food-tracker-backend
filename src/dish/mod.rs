use aide::axum::{
    routing::{get, post},
    ApiRouter,
};

use crate::state::AppState;

mod delete;
mod delete_warning;
mod get;
mod ingredient;
mod list;
mod post;

pub fn route(state: AppState) -> ApiRouter {
    ApiRouter::new()
        .api_route("/", post(post::post_dish).get(list::list_dish))
        .api_route("/:dish_id", get(get::get_dish).delete(delete::delete_dish))
        .api_route(
            "/:dish_id/delete-warning",
            get(delete_warning::get_delete_warning),
        )
        .nest_api_service("/:dish_id/ingredient", ingredient::route(state.clone()))
        .with_state(state)
}
