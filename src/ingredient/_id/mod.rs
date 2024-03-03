use aide::axum::{routing::get_with, ApiRouter};

use crate::state::AppState;

use self::get::get_ingredient_docs;

mod get;
mod properties;

pub fn route(state: AppState) -> ApiRouter {
    ApiRouter::new()
        .api_route("/", get_with(get::get_ingredient, get_ingredient_docs))
        .nest_api_service("/properties", properties::route(state.clone()))
        .with_state(state)
}
