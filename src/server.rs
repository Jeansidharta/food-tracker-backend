use std::path::PathBuf;

use aide::{
    axum::{routing::get, ApiRouter, IntoApiResponse},
    openapi::{Info, OpenApi},
    redoc::Redoc,
};
use axum::{
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    Extension, Json,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::migrate::MigrateDatabase;

#[derive(Deserialize, Serialize, Default, JsonSchema)]
pub struct ServerResponse<T> {
    pub message: String,
    pub data: Option<T>,
    #[serde(skip)]
    pub status_code: StatusCode,
}

impl<T> ServerResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            message: "success".to_string(),
            data: Some(data),
            status_code: StatusCode::OK,
        }
    }

    pub fn json(self) -> Json<Self> {
        Json(self)
    }
}

impl ServerResponse<()> {
    pub fn error(message: impl ToString) -> Self {
        ServerResponse {
            data: None,
            message: message.to_string(),
            status_code: StatusCode::BAD_REQUEST,
        }
    }
}

impl<T: Serialize> IntoResponse for ServerResponse<T> {
    fn into_response(self) -> Response {
        (self.status_code, serde_json::to_string(&self).unwrap()).into_response()
    }
}

pub type ServerResponseResult<T> = Result<Json<ServerResponse<T>>, AppError>;

use crate::{
    app_error::AppError, dish::route as route_dish, ingredient::route as route_ingredient,
    meal::route as route_meal, state::AppState,
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

pub async fn server(port: u16, database_path: Option<PathBuf>) {
    let mut api = OpenApi {
        info: Info {
            description: Some("API for the Food Tracker app".to_string()),
            ..Info::default()
        },
        ..OpenApi::default()
    };

    let database_url = database_path
        .map(|path| format!("sqlite:{}", path.to_string_lossy()))
        .or_else(|| std::env::var("DATABASE_URL").ok())
        .or_else(|| {
            dirs::state_dir().map(|d| format!("sqlite:{}/foodtracker.sqlite3", d.to_string_lossy()))
        })
        .expect("Could not resolve database path");

    if !sqlx::sqlite::Sqlite::database_exists(&database_url)
        .await
        .unwrap()
    {
        sqlx::sqlite::Sqlite::create_database(&database_url)
            .await
            .unwrap();
    }

    println!("Connecting to database at {}", database_url);

    let state = AppState {
        connection: sqlx::sqlite::SqlitePool::connect(&database_url)
            .await
            .unwrap(),
    };

    sqlx::migrate!("./migrations")
        .run(&state.connection)
        .await
        .unwrap();

    let app = ApiRouter::new()
        .nest_api_service("/ingredient", route_ingredient(state.clone()))
        .nest_api_service("/dish", route_dish(state.clone()))
        .nest_api_service("/meal", route_meal(state.clone()))
        .route("/openapi.json", get(serve_api))
        .route("/docs", Redoc::new("/openapi.json").axum_route())
        .layer(middleware::from_fn(logging_middleware))
        .layer(tower_http::cors::CorsLayer::permissive())
        .with_state(state);

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
