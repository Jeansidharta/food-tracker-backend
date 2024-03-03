use aide::transform::TransformOperation;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    app_error::InternalServerError,
    server::{ServerResponse, ServerResponseResult},
    state::AppState,
};

#[derive(JsonSchema, Deserialize)]
pub struct IngredientId {
    pub ingredient_id: i64,
}

#[derive(Deserialize, JsonSchema)]
pub struct PostPropertiesBody {
    pub product_code: String,
}

#[derive(Serialize, thiserror::Error, Debug)]
pub enum PostIngredientPropertiesError {
    #[error("The product code should not be empty")]
    ProductCodeIsEmpty(String),
    #[error("Open Food Facts returned {0}: \"{1}\"")]
    HttpStatusNotSuccess(String, String),
    #[error("Open Food Facts could not find the product code {0}")]
    ProductCodeNotFound(String),
}

#[derive(Deserialize)]
struct FoodFactsResult {
    status: i64,
}

async fn fetch_from_open_food_facts(product_code: &String) -> Result<String, anyhow::Error> {
    if product_code.is_empty() {
        return Err(PostIngredientPropertiesError::ProductCodeIsEmpty(product_code.clone()).into());
    }

    let response = reqwest::RequestBuilder::from_parts(
        reqwest::Client::new(),
        reqwest::Request::new(
            reqwest::Method::GET,
            reqwest::Url::parse(&format!(
                "https://world.openfoodfacts.org/api/v2/product/{}.json",
                product_code
            ))
            .unwrap(),
        ),
    )
    .header("Contact", "jeansidharta@gmail.com")
    .header(
        "App",
        "https://github.com/Jeansidharta/food-tracker-frontend",
    )
    .send()
    .await?;

    let status = response.status();
    let text = response.text().await?;

    if !status.is_success() {
        return Err(
            PostIngredientPropertiesError::HttpStatusNotSuccess(status.to_string(), text).into(),
        );
    }

    let json = serde_json::from_str::<FoodFactsResult>(&text)?;

    if json.status == 0 {
        return Err(
            PostIngredientPropertiesError::ProductCodeNotFound(product_code.clone()).into(),
        );
    }

    Ok(text)
}

#[derive(JsonSchema, Serialize)]
pub struct PostIngredientPropertiesResult {
    ingredient_id: i64,
    product_code: String,
    kcal_100g: Option<i64>,
}

pub async fn post_ingredient_properties(
    State(AppState { connection }): State<AppState>,
    Path(IngredientId { ingredient_id }): Path<IngredientId>,
    Json(PostPropertiesBody { product_code }): Json<PostPropertiesBody>,
) -> ServerResponseResult<PostIngredientPropertiesResult> {
    let food_facts = fetch_from_open_food_facts(&product_code).await?;
    let data = sqlx::query_as!(
        PostIngredientPropertiesResult,
        r#"
        INSERT OR REPLACE INTO IngredientProperties (
            ingredient_id,
            product_code,
            open_food_facts_json
        ) VALUES (?, ?, ?)
        RETURNING
            ingredient_id,
            product_code,
            kcal_100g
        ;"#,
        ingredient_id,
        product_code,
        food_facts
    )
    .fetch_one(&connection)
    .await?;

    Ok(ServerResponse::success_code(data, StatusCode::CREATED).json())
}

pub fn post_ingredient_properties_docs(op: TransformOperation) -> TransformOperation {
    op.response::<201, Json<ServerResponse<PostIngredientPropertiesResult>>>()
        .response::<500, Json<InternalServerError>>()
}
