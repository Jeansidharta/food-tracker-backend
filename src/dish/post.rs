use axum::{extract::State, Json};
use schemars::JsonSchema;
use serde::Deserialize;
use sqlx::QueryBuilder;
use thiserror::Error;

use crate::{
    get_missing_items,
    models::{Dish, DishIngredient},
    server::{ServerResponse, ServerResponseResult},
    state::AppState,
};

#[derive(Deserialize, JsonSchema, Debug)]
struct PostDishIngredient {
    weight: i64,
    ingredient_id: i64,
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct PostDish {
    name: String,
    prep_date: Option<i64>,
    dish_ingredients: Option<Vec<PostDishIngredient>>,
    total_weight: Option<i64>,
}

#[derive(Error, Debug)]
enum PostDishError {
    #[error("The following ingredients don't exist: {0:?}")]
    UnknownIngredientId(Vec<i64>),
}
pub async fn post_dish(
    State(AppState { connection }): State<AppState>,
    Json(PostDish {
        total_weight,
        name,
        prep_date,
        dish_ingredients,
    }): Json<PostDish>,
) -> ServerResponseResult<(Dish, Vec<DishIngredient>)> {
    let dish_ingredients = dish_ingredients.unwrap_or_default();

    if !dish_ingredients.is_empty() {
        let unknown_ingredients = {
            let ids = dish_ingredients
                .iter()
                .map(|item| item.ingredient_id.to_string())
                .collect::<Vec<String>>()
                .join(", ");

            let ingredients_in_database =
                sqlx::QueryBuilder::new("SELECT id FROM Ingredient WHERE id IN (")
                    .push(ids)
                    .push(")")
                    .build_query_scalar::<i64>()
                    .fetch_all(&connection)
                    .await?;

            get_missing_items(
                ingredients_in_database,
                dish_ingredients.iter().map(|i| i.ingredient_id),
            )
        };

        if !unknown_ingredients.is_empty() {
            return Err(PostDishError::UnknownIngredientId(unknown_ingredients))?;
        }
    }

    let transaction = connection.begin().await?;

    let total_weight = total_weight.unwrap_or(0);

    let new_dish = sqlx::query_as!(
        Dish,
        "INSERT INTO Dish
            (name, prep_date, total_weight)
        VALUES
            (?, ?, ?)
        RETURNING id, creation_date, prep_date, name, total_weight;",
        name,
        prep_date,
        total_weight
    )
    .fetch_one(&connection)
    .await?;

    let new_dish_ingredients = if !dish_ingredients.is_empty() {
        QueryBuilder::new("INSERT INTO DishIngredient (dish_id, ingredient_id, weight) ")
            .push_values(dish_ingredients, |mut b, dish_ingredient| {
                b.push_bind(new_dish.id)
                    .push_bind(dish_ingredient.ingredient_id)
                    .push_bind(dish_ingredient.weight);
            })
            .push(
                r#"
            ON CONFLICT DO
            UPDATE SET weight = DishIngredient.weight + excluded.weight
            RETURNING dish_id, ingredient_id, weight, creation_date;"#,
            )
            .build_query_as::<DishIngredient>()
            .fetch_all(&connection)
            .await?
    } else {
        vec![]
    };

    transaction.commit().await?;

    Ok(ServerResponse::success((new_dish, new_dish_ingredients)).json())
}
