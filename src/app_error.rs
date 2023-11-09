use aide::OperationOutput;
use axum::response::{IntoResponse, Response};

use crate::server::ServerResponse;

pub struct AppError(pub anyhow::Error);

impl OperationOutput for AppError {
    type Inner = anyhow::Error;
}

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        {
            ServerResponse::error(self.0).into_response()
        }
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
