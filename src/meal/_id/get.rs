use axum::extract::{Path, State};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    models::Meal,
    server::{ServerResponse, ServerResponseResult},
    state::AppState,
};

#[derive(Serialize, JsonSchema)]
pub struct MealComponent {
    weight: i64,
    name: Option<String>,
    id: i64,
    kcal: Option<i64>,
    proteins: Option<i64>,
    fat: Option<i64>,
    carbohydrates: Option<i64>,
}

#[derive(Serialize, JsonSchema)]
pub struct GetMealResponse {
    meal: Meal,
    dishes: Vec<MealComponent>,
    ingredients: Vec<MealComponent>,
}

#[derive(Error, Debug)]
enum GetMeal {
    #[error("Could not find meal with id \"{0}\"")]
    MealNotFound(i64),
}

#[derive(Deserialize, JsonSchema)]
pub struct MealId {
    meal_id: i64,
}

async fn get_meal_table(
    connection: &sqlx::Pool<sqlx::Sqlite>,
    meal_id: i64,
) -> anyhow::Result<Meal> {
    Ok(sqlx::query_as!(
        Meal,
        r#"
        SELECT
            id,
            creation_date,
            duration,
            description,
            eat_date
        FROM Meal
        WHERE id = ?;"#,
        meal_id
    )
    .fetch_optional(connection)
    .await?
    .ok_or(GetMeal::MealNotFound(meal_id))?)
}

async fn get_meal_ingredients_table(
    connection: &sqlx::Pool<sqlx::Sqlite>,
    meal_id: i64,
) -> anyhow::Result<Vec<MealComponent>> {
    Ok(sqlx::query_as!(
        MealComponent,
        r#"
        SELECT 
            MealIngredient.weight,
            Ingredient.name as name,
            Ingredient.id as id,
            iif(kcal_100g IS NOT NULL, CAST ((kcal_100g * MealIngredient.weight / 100) AS INTEGER), NULL) as kcal,
            iif(fat_100g IS NOT NULL, CAST ((fat_100g * MealIngredient.weight / 100) AS INTEGER), NULL) as fat,
            iif(carbohydrates_100g IS NOT NULL, CAST ((carbohydrates_100g * MealIngredient.weight / 100) AS INTEGER), NULL) as carbohydrates,
            iif(proteins_100g IS NOT NULL, CAST ((proteins_100g * MealIngredient.weight / 100) AS INTEGER), NULL) as proteins
        FROM Meal
            JOIN MealIngredient ON Meal.id = MealIngredient.meal_id
            JOIN Ingredient ON MealIngredient.ingredient_id = Ingredient.id
            LEFT JOIN IngredientProperties ON IngredientProperties.ingredient_id = Ingredient.id
        WHERE Meal.id = ?;
        "#,
        meal_id
    )
    .fetch_all(connection)
    .await?)
}

mod meal_dishes {
    use std::collections::HashMap;

    use schemars::JsonSchema;
    use serde::Serialize;
    use sqlx::Sqlite;

    use super::MealComponent;

    #[derive(Serialize, JsonSchema)]
    pub struct DatabaseDish {
        weight: f64,
        dish_total_weight: Option<f64>,
        name: Option<String>,
        id: i64,
    }

    #[derive(Serialize, JsonSchema, sqlx::FromRow)]
    pub struct DatabaseDishIngredient {
        dish_id: i64,
        ingredient_weight: f64,
        kcal_100g: Option<f64>,
        proteins_100g: Option<f64>,
        fat_100g: Option<f64>,
        carbohydrates_100g: Option<f64>,
    }

    pub async fn get_meal_dishes_table(
        connection: &sqlx::Pool<sqlx::Sqlite>,
        meal_id: i64,
    ) -> anyhow::Result<Vec<MealComponent>> {
        let dishes = sqlx::query_as!(
            DatabaseDish,
            r#"
        SELECT 
            (CAST (MealDish.weight AS FLOAT)) AS weight,
            (CAST (Dish.total_weight AS FLOAT)) AS dish_total_weight,
            Dish.name,
            Dish.id
        FROM Meal
            JOIN MealDish ON Meal.id = MealDish.meal_id
            JOIN Dish ON MealDish.dish_id = Dish.id
        WHERE Meal.id = ?;
        "#,
            meal_id
        )
        .fetch_all(connection)
        .await?;

        let mut dish_ingredients_dict = sqlx::QueryBuilder::<Sqlite>::new(
            r#"
            SELECT
                iif(kcal_100g IS NOT NULL, CAST (kcal_100g AS FLOAT), NULL) AS kcal_100g,
                iif(proteins_100g IS NOT NULL, CAST (proteins_100g AS FLOAT), NULL) AS proteins_100g,
                iif(fat_100g IS NOT NULL, CAST (fat_100g AS FLOAT), NULL) AS fat_100g,
                iif(carbohydrates_100g IS NOT NULL, CAST (carbohydrates_100g AS FLOAT), NULL) AS carbohydrates_100g,
                DishIngredient.dish_id,
                CAST (DishIngredient.weight AS FLOAT) AS ingredient_weight
            FROM
                DishIngredient
                LEFT JOIN Ingredient ON Ingredient.id = DishIngredient.ingredient_id
                LEFT JOIN IngredientProperties ON Ingredient.id = IngredientProperties.ingredient_id
            WHERE DishIngredient.dish_id IN 
            "#,
        )
        .push_tuples(dishes.iter(), |mut p, dish| {
            p.push_bind(dish.id);
        })
        .build_query_as::<DatabaseDishIngredient>()
        .fetch_all(connection)
        .await?
        .into_iter()
        .fold(
            HashMap::<i64, Vec<DatabaseDishIngredient>>::new(),
            |mut dict, ingredient| {
                dict.entry(ingredient.dish_id).or_default().push(ingredient);
                dict
            },
        );

        Ok(dishes
            .into_iter()
            .map(|dish| {
                let ingredients = dish_ingredients_dict.remove(&dish.id).unwrap_or_default();
                let dish_total_weight = dish
                    .dish_total_weight
                    .unwrap_or_else(|| ingredients.iter().map(|i| i.ingredient_weight).sum());
                let nutrients = ingredients
                    .into_iter()
                    .map(|i| {
                        (
                            i.kcal_100g.unwrap_or_default() * i.ingredient_weight
                                / dish_total_weight,
                            i.proteins_100g.unwrap_or_default() * i.ingredient_weight
                                / dish_total_weight,
                            i.fat_100g.unwrap_or_default() * i.ingredient_weight
                                / dish_total_weight,
                            i.carbohydrates_100g.unwrap_or_default() * i.ingredient_weight
                                / dish_total_weight,
                        )
                    })
                    .reduce(|(a1, b1, c1, d1), (a2, b2, c2, d2)| {
                        ((a1 + a2), (b1 + b2), (c1 + c2), (d1 + d2))
                    })
                    .map(|(kcal_100g, proteins_100g, fat_100g, carbohydrates_100g)| {
                        (
                            kcal_100g * dish.weight / 100.0,
                            proteins_100g * dish.weight / 100.0,
                            fat_100g * dish.weight / 100.0,
                            carbohydrates_100g * dish.weight / 100.0,
                        )
                    });

                MealComponent {
                    weight: dish.weight as i64,
                    name: dish.name,
                    id: dish.id,
                    kcal: nutrients.map(|n| n.0 as i64),
                    proteins: nutrients.map(|n| n.1 as i64),
                    fat: nutrients.map(|n| n.2 as i64),
                    carbohydrates: nutrients.map(|n| n.3 as i64),
                }
            })
            .collect())
    }
}

pub async fn get_meal(
    State(AppState { connection }): State<AppState>,
    Path(MealId { meal_id }): Path<MealId>,
) -> ServerResponseResult<GetMealResponse> {
    let (meal, dishes, ingredients) = futures::try_join!(
        get_meal_table(&connection, meal_id),
        meal_dishes::get_meal_dishes_table(&connection, meal_id),
        get_meal_ingredients_table(&connection, meal_id),
    )?;

    Ok(ServerResponse::success(GetMealResponse {
        meal,
        dishes,
        ingredients,
    })
    .json())
}
