use axum::extract::State;
use schemars::JsonSchema;
use serde::Serialize;

use crate::{
    server::{ServerResponse, ServerResponseResult},
    state::AppState,
};

#[derive(Serialize, JsonSchema)]
pub struct MealComponent {
    id: i64,
    name: Option<String>,
    component_type: String,
}

#[derive(Serialize, JsonSchema)]
pub struct GetComponentResult {
    components: Vec<MealComponent>,
}

pub async fn get_component(
    State(AppState { connection }): State<AppState>,
) -> ServerResponseResult<GetComponentResult> {
    let components = sqlx::query_as!(
        MealComponent,
        r#"
        SELECT id, name, 'Ingredient' as component_type from Ingredient
        UNION
        SELECT id, name, 'Dish' as component_type from Dish;
        "#
    )
    .fetch_all(&connection)
    .await?;

    Ok(ServerResponse::success(GetComponentResult { components }).json())
}
