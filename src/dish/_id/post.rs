use std::{future::Future, pin::Pin};

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
            name = ?, prep_date = ?, total_weight = ?
        WHERE id = ?
        RETURNING id as "id!", creation_date, prep_date, name, total_weight;"#,
        name,
        prep_date,
        total_weight,
        id
    )
    .fetch_one(&connection)
    .await?;
    let insert_dish_futures = dish_ingredients
        .iter()
        .map(
            |ingredient| -> Pin<Box<dyn Future<Output = anyhow::Result<DishIngredient>> + Send>> {
                let connection = connection.clone();
                Box::pin(async move {
                    let current_ingredient = sqlx::query_scalar!(
                        "SELECT dish_id FROM DishIngredient WHERE dish_id = ? AND ingredient_id = ?;",
                        id,
                        ingredient.ingredient_id
                    )
                    .fetch_optional(&connection)
                    .await?;

                    Ok(if current_ingredient.is_some() {
                        sqlx::query_as!(
                            DishIngredient,
                            r#"
                            UPDATE DishIngredient
                                SET weight = ?
                            WHERE dish_id = ? AND ingredient_id = ?
                            RETURNING dish_id, ingredient_id, weight, creation_date;"#,
                            ingredient.weight,
                            id,
                            ingredient.ingredient_id
                        )
                        .fetch_one(&connection)
                        .await?
                    } else {
                        sqlx::query_as!(
                            DishIngredient,
                            r#"
                            INSERT INTO DishIngredient
                                (dish_id, ingredient_id, weight)
                            VALUES (?, ?, ?)
                            RETURNING dish_id, ingredient_id, weight, creation_date;"#,
                            id,
                            ingredient.ingredient_id,
                            ingredient.weight,
                        )
                        .fetch_one(&connection)
                        .await?
                    })
                })
            },
        )
        .collect::<Vec<Pin<Box<dyn Future<Output = anyhow::Result<DishIngredient>> + Send>>>>();

    let new_dish_ingredients = futures::future::join_all(insert_dish_futures)
        .await
        .into_iter()
        .collect::<anyhow::Result<Vec<DishIngredient>>>()?;

    let mut query = sqlx::QueryBuilder::new(
        r#"
        DELETE FROM DishIngredient
        WHERE
            ingredient_id IN
                (SELECT ingredient_id FROM DishIngredient where dish_id = 2)
            AND NOT ingredient_id IN ("#,
    );
    dish_ingredients.into_iter().for_each(|i| {
        query.push(i.ingredient_id);
    });
    query.push(");").build().execute(&connection).await?;

    transaction.commit().await?;

    Ok(ServerResponse::success((new_dish, new_dish_ingredients)).json())
}
