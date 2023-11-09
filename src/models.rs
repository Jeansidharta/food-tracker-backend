use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, PartialEq, PartialOrd, JsonSchema)]
pub struct Ingredient {
    pub id: i64,
    pub creation_date: i64,
    pub name: String,
}

#[derive(PartialEq, PartialOrd, Deserialize, JsonSchema)]
pub struct NewIngredient {
    pub name: String,
}

#[derive(Serialize, JsonSchema)]
pub struct Dish {
    pub id: i64,
    pub creation_date: i64,
    pub prep_date: Option<i64>,
    pub name: Option<String>,
}

#[derive(sqlx::FromRow, Serialize, JsonSchema)]
pub struct DishIngredient {
    pub creation_date: i64,
    pub dish_id: i64,
    pub ingredient_id: i64,
    pub weight: i64,
}

#[derive(JsonSchema)]
pub struct NewDish {
    pub name: Option<String>,
    pub prep_date: Option<i64>,
}

#[derive(JsonSchema)]
pub struct NewDishIngredient {
    pub dish_id: i64,
    pub ingredient_id: i64,
    pub weight: i64,
}

#[derive(Serialize, Default, JsonSchema)]
pub struct Meal {
    pub id: i64,
    pub creation_date: i64,
    pub eat_date: Option<i64>,
    pub duration: Option<i64>,
    pub description: Option<String>,
    pub hunger_level: Option<i64>,
    pub desire_to_eat: Option<i64>,
    pub fullness_afterwards: Option<i64>,
}

#[derive(Serialize, sqlx::FromRow, Default, JsonSchema)]
pub struct MealDish {
    pub creation_date: i64,
    pub dish_id: i64,
    pub meal_id: i64,
    pub weight: i64,
}

#[derive(Deserialize, Default, JsonSchema)]
pub struct NewMeal {
    pub eat_date: Option<i64>,
    pub duration: Option<i64>,
    pub description: Option<String>,
    pub hunger_level: Option<i64>,
    pub desire_to_eat: Option<i64>,
    pub fullness_afterwards: Option<i64>,
}

#[derive(Default, JsonSchema)]
pub struct NewMealDish {
    pub dish_id: i64,
    pub meal_id: i64,
    pub weight: i64,
}
