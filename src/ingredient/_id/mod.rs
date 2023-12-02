use aide::axum::{routing::get, ApiRouter};

use crate::state::AppState;

mod get;
mod properties;

pub fn route(state: AppState) -> ApiRouter {
    ApiRouter::new()
        .api_route("/", get(get::get_ingredient))
        .nest_api_service("/properties", properties::route(state.clone()))
        .with_state(state)
}
