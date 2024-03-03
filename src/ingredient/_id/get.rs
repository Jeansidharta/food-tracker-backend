use aide::transform::TransformOperation;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use thiserror::Error;

use crate::{
    app_error::InternalServerError,
    server::{ServerResponse, ServerResponseResult},
    state::AppState,
};

#[derive(Error, Debug)]
enum GetIngredient {
    #[error("Could not find ingredient with id \"{0}\"")]
    IngredientNotFound(i64),
}

#[derive(JsonSchema, Deserialize)]
pub struct IngredientId {
    pub ingredient_id: i64,
}

#[derive(JsonSchema, Serialize)]
pub struct IngredientResult {
    id: i64,
    name: String,
    creation_date: i64,
}

#[derive(JsonSchema, Serialize)]
pub struct IngredientPropertiesResult {
    kcal_100g: Option<i64>,
    product_name: Option<String>,
    product_code: String,
    carbohydrates_100g: Option<i64>,
    proteins_100g: Option<i64>,
    fat_100g: Option<i64>,
}

#[derive(JsonSchema, Serialize)]
pub struct GetIngredientResult {
    ingredient: IngredientResult,
    ingredient_properties: Option<IngredientPropertiesResult>,
}

async fn fetch_ingredient(
    connection: &Pool<Sqlite>,
    ingredient_id: i64,
) -> Result<IngredientResult, InternalServerError> {
    let ingredient = sqlx::query_as!(
        IngredientResult,
        r#"
        SELECT
            name,
            id,
            creation_date
        FROM Ingredient
        WHERE id=?"#,
        ingredient_id
    )
    .fetch_optional(connection)
    .await?
    .ok_or(GetIngredient::IngredientNotFound(ingredient_id))?;

    Ok(ingredient)
}

async fn fetch_ingredient_properties(
    connection: &Pool<Sqlite>,
    ingredient_id: i64,
) -> Result<Option<IngredientPropertiesResult>, InternalServerError> {
    let ingredient_properties = sqlx::query_as!(
        IngredientPropertiesResult,
        r#"
        SELECT
            kcal_100g,
            fat_100g,
            carbohydrates_100g,
            proteins_100g,
            product_name,
            product_code
        FROM IngredientProperties
        WHERE ingredient_id=?"#,
        ingredient_id
    )
    .fetch_optional(connection)
    .await?;

    Ok(ingredient_properties)
}

pub async fn get_ingredient(
    State(AppState { connection }): State<AppState>,
    Path(IngredientId { ingredient_id }): Path<IngredientId>,
) -> ServerResponseResult<GetIngredientResult> {
    let (ingredient, ingredient_properties) = futures::join!(
        fetch_ingredient(&connection, ingredient_id),
        fetch_ingredient_properties(&connection, ingredient_id)
    );

    let ingredient = ingredient?;
    let ingredient_properties = ingredient_properties?;

    Ok(ServerResponse::success_code(
        GetIngredientResult {
            ingredient,
            ingredient_properties,
        },
        StatusCode::OK,
    )
    .json())
}

pub fn get_ingredient_docs(op: TransformOperation) -> TransformOperation {
    op.response::<200, Json<ServerResponse<GetIngredientResult>>>()
        .response::<500, Json<InternalServerError>>()
}
