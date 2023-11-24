use axum::{
    extract::{Path, State},
    Json,
};
use schemars::JsonSchema;
use serde::Deserialize;
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
    is_finished: Option<bool>,
}

#[derive(Error, Debug)]
enum PostDishError {
    #[error("The following ingredients don't exist: {0:?}")]
    UnknownIngredientId(Vec<i64>),
}

#[derive(Deserialize, JsonSchema)]
pub struct DishId {
    dish_id: i64,
}

pub async fn post_edit_dish(
    State(AppState { connection }): State<AppState>,
    Path(DishId { dish_id: id }): Path<DishId>,
    Json(PostDish {
        total_weight,
        name,
        prep_date,
        dish_ingredients,
        is_finished,
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
        r#"UPDATE Dish SET
            name = ?,
            prep_date = ?,
            is_finished = ?,
            total_weight = ?
        WHERE id = ?
        RETURNING
            id as "id!",
            creation_date,
            prep_date,
            name,
            total_weight,
            is_finished;
        "#,
        name,
        prep_date,
        is_finished,
        total_weight,
        id
    )
    .fetch_one(&connection)
    .await?;

    let new_dish_ingredients = if dish_ingredients.is_empty() {
        vec![]
    } else {
        sqlx::QueryBuilder::new(
            r#"
            DELETE FROM DishIngredient WHERE dish_id = "#,
        )
        .push_bind(id)
        .push(
            r#";
            INSERT INTO DishIngredient
                (dish_id, ingredient_id, weight)"#,
        )
        .push_values(dish_ingredients.iter(), |mut b, ingredient| {
            b.push_bind(id)
                .push_bind(ingredient.ingredient_id)
                .push_bind(ingredient.weight);
        })
        .push(
            r#"
            ON CONFLICT DO UPDATE SET weight = DishIngredient.weight + excluded.weight
            RETURNING dish_id, ingredient_id, weight, creation_date;"#,
        )
        .build_query_as::<DishIngredient>()
        .fetch_all(&connection)
        .await?
    };

    transaction.commit().await?;

    Ok(ServerResponse::success((new_dish, new_dish_ingredients)).json())
}
