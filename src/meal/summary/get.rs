use axum::extract::{Query, State};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Sqlite};
use thiserror::Error;

use crate::{
    models::Meal,
    server::{ServerResponse, ServerResponseResult},
    state::AppState,
};

#[derive(Serialize, JsonSchema, FromRow, Clone)]
pub struct MealComponent {
    weight: i64,
    name: Option<String>,
    id: i64,
    kcal: Option<i64>,
    proteins: Option<i64>,
    fat: Option<i64>,
    carbohydrates: Option<i64>,
}

#[derive(Serialize, JsonSchema, Clone)]
pub struct GetMealResponse {
    dishes: Vec<MealComponent>,
    ingredients: Vec<MealComponent>,
}

#[derive(Error, Debug)]
enum GetMeal {
    #[error("Could not find meal with id \"{0}\"")]
    MealNotFound(i64),
}

#[derive(Deserialize, JsonSchema)]
pub struct EatenSince {
    eaten_since: i64,
}

async fn get_meal_table(
    connection: &sqlx::Pool<sqlx::Sqlite>,
    eaten_since: i64,
) -> anyhow::Result<Vec<Meal>> {
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
        WHERE eat_date > ?;"#,
        eaten_since
    )
    .fetch_all(connection)
    .await?)
}

mod meal_dishes {
    use std::collections::HashMap;

    use schemars::JsonSchema;
    use serde::Serialize;
    use sqlx::{prelude::FromRow, Sqlite};

    use crate::models::Meal;

    use super::MealComponent;

    #[derive(Serialize, JsonSchema, FromRow)]
    pub struct DatabaseDish {
        weight: f64,
        dish_total_weight: Option<f64>,
        name: Option<String>,
        meal_id: i64,
        dish_id: i64,
    }

    #[derive(Serialize, JsonSchema, sqlx::FromRow, Clone)]
    pub struct DatabaseDishIngredient {
        dish_id: i64,
        meal_id: i64,
        ingredient_weight: f64,
        kcal_100g: Option<f64>,
        proteins_100g: Option<f64>,
        fat_100g: Option<f64>,
        carbohydrates_100g: Option<f64>,
    }

    pub async fn get_meal_dishes_table(
        connection: &sqlx::Pool<sqlx::Sqlite>,
        meals: &[Meal],
    ) -> anyhow::Result<Vec<MealComponent>> {
        let dishes = sqlx::QueryBuilder::<Sqlite>::new(
            r#"
        SELECT 
            (CAST (MealDish.weight AS FLOAT)) AS weight,
            (CAST (Dish.total_weight AS FLOAT)) AS dish_total_weight,
            Dish.name,
            Dish.id as dish_id,
            Meal.id as meal_id
        FROM Meal
            JOIN MealDish ON Meal.id = MealDish.meal_id
            JOIN Dish ON MealDish.dish_id = Dish.id
        WHERE Meal.id IN ?;
        "#,
        )
        .push_tuples(meals.iter(), |mut p, meal| {
            p.push_bind(meal.id);
        })
        .build_query_as::<DatabaseDish>()
        .fetch_all(connection)
        .await?;

        let (mut meal_ingredient_dict, dish_ingredients_dict)= sqlx::QueryBuilder::<Sqlite>::new(
            r#"
            SELECT
                iif(kcal_100g IS NOT NULL, CAST (kcal_100g AS FLOAT), NULL) AS kcal_100g,
                iif(proteins_100g IS NOT NULL, CAST (proteins_100g AS FLOAT), NULL) AS proteins_100g,
                iif(fat_100g IS NOT NULL, CAST (fat_100g AS FLOAT), NULL) AS fat_100g,
                iif(carbohydrates_100g IS NOT NULL, CAST (carbohydrates_100g AS FLOAT), NULL) AS carbohydrates_100g,
                DishIngredient.dish_id,
                MealDish.meal_id,
                CAST (DishIngredient.weight AS FLOAT) AS ingredient_weight
            FROM
                DishIngredient
                LEFT JOIN Dish ON Dish.id = DishIngredient.dish_id
                LEFT JOIN MealDish ON MealDish.dish_id = Dish.id
                LEFT JOIN Ingredient ON Ingredient.id = DishIngredient.ingredient_id
                LEFT JOIN IngredientProperties ON Ingredient.id = IngredientProperties.ingredient_id
            WHERE MealDish.id IN
            "#
        )
        .push_tuples(dishes.iter(), |mut p, dish| {
             p.push_bind(dish.dish_id);
        })
        .build_query_as::<DatabaseDishIngredient>()
        .fetch_all(connection)
        .await?
        .into_iter()
        .fold(
            (HashMap::<i64, Vec<DatabaseDishIngredient>>::new(),HashMap::<i64, Vec<DatabaseDishIngredient>>::new()),
            |(mut mealDict, mut dishDict), ingredient| {
                    let meal_id = ingredient.meal_id;
                    let dish_id = ingredient.dish_id;
                mealDict.entry(meal_id).or_default().push(ingredient.clone());
                dishDict.entry(dish_id).or_default().push(ingredient);
                (mealDict,dishDict)
            },
        );

        let dishes_dict = dishes.iter().fold(
            HashMap::<i64, Vec<&DatabaseDish>>::new(),
            |mut dict, dish| {
                dict.entry(dish.meal_id).or_default().push(dish);
                dict
            },
        );

        Ok(meals
            .iter()
            .map(|meal| {
                let ingredients = meal_ingredient_dict.remove(&meal.id).unwrap_or_default();
                let meal_total_weight: f64 = dishes_dict
                    .get(&meal.id)
                    .unwrap()
                    .iter()
                    .map(|d| {
                        d.dish_total_weight.unwrap_or_else(|| {
                            dish_ingredients_dict
                                .get(&d.dish_id)
                                .unwrap()
                                .iter()
                                .map(|i| i.ingredient_weight)
                                .sum()
                        })
                    })
                    .sum();

                let nutrients = ingredients
                    .into_iter()
                    .map(|i| {
                        (
                            i.kcal_100g.unwrap_or_default() * i.ingredient_weight
                                / meal_total_weight,
                            i.proteins_100g.unwrap_or_default() * i.ingredient_weight
                                / meal_total_weight,
                            i.fat_100g.unwrap_or_default() * i.ingredient_weight
                                / meal_total_weight,
                            i.carbohydrates_100g.unwrap_or_default() * i.ingredient_weight
                                / meal_total_weight,
                        )
                    })
                    .reduce(|(a1, b1, c1, d1), (a2, b2, c2, d2)| {
                        ((a1 + a2), (b1 + b2), (c1 + c2), (d1 + d2))
                    });

                MealComponent {
                    meal_id: meal.id,
                    weight: meal_total_weight as i64,
                    name: meal.description.clone(),
                    id: meal.id,
                    kcal: nutrients.map(|n| n.0 as i64),
                    proteins: nutrients.map(|n| n.1 as i64),
                    fat: nutrients.map(|n| n.2 as i64),
                    carbohydrates: nutrients.map(|n| n.3 as i64),
                }
            })
            .collect())
    }
}

pub async fn get_summary(
    State(AppState { connection }): State<AppState>,
    Query(EatenSince { eaten_since }): Query<EatenSince>,
) -> ServerResponseResult<Vec<GetMealResponse>> {
    let meals = get_meal_table(&connection, eaten_since).await?;
    let dishes = meal_dishes::get_meal_dishes_table(&connection, &meals).await?;

    let meals = meals
        .into_iter()
        .map(|meal| MealComponent {
            dishes: dishes
                .iter()
                .filter(|d| d.meal_id == meal.id)
                .cloned()
                .collect::<Vec<MealComponent>>(),
            ingredients: ingredients
                .iter()
                .filter(|i| i.meal_id == meal.id)
                .cloned()
                .collect::<Vec<MealComponent>>(),
        })
        .collect();

    Ok(ServerResponse::success(GetMealResponse {}).json())
}
