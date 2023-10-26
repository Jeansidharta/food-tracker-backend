use aide::{
    axum::{routing::get, ApiRouter, IntoApiResponse},
    openapi::{Info, OpenApi},
    redoc::Redoc,
};
use axum::{
    http::{HeaderValue, Request},
    middleware::{self, Next},
    response::Response,
    Extension, Json,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, JsonSchema)]
pub struct ServerResponse<T> {
    pub message: String,
    pub data: T,
}

use crate::{
    dish::route as route_dish, ingredient::route as route_ingredient, meal::route as route_meal,
};

async fn logging_middleware<B>(request: Request<B>, next: Next<B>) -> Response {
    let now = std::time::Instant::now();

    let response = next.run(request).await;

    let elapsed_time = now.elapsed();
    println!("Request finished in {}µs", elapsed_time.as_micros());

    response
}

async fn serve_api(Extension(api): Extension<OpenApi>) -> impl IntoApiResponse {
    Json(api)
}
pub async fn server(port: u16) {
    let mut api = OpenApi {
        info: Info {
            description: Some("API for the Food Tracker app".to_string()),
            ..Info::default()
        },
        ..OpenApi::default()
    };

    let app = ApiRouter::new()
        .nest_api_service("/ingredient", route_ingredient())
        .nest_api_service("/dish", route_dish())
        .nest_api_service("/meal", route_meal())
        .route("/openapi.json", get(serve_api))
        .route("/docs", Redoc::new("/openapi.json").axum_route())
        .layer(middleware::from_fn(logging_middleware))
        .layer(tower_http::cors::CorsLayer::permissive());

    println!("Server running on port {port}");

    axum::Server::bind(&std::net::SocketAddr::V4(std::net::SocketAddrV4::new(
        std::net::Ipv4Addr::new(127, 0, 0, 1),
        port,
    )))
    .serve(
        app.finish_api(&mut api)
            .layer(Extension(api))
            .into_make_service(),
    )
    .await
    .unwrap();
}
