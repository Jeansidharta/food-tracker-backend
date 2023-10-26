#![allow(non_snake_case)]
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, PartialEq, PartialOrd, Serialize, JsonSchema)]
#[diesel(table_name = crate::schema::Ingredient)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Ingredient {
    pub id: i32,
    pub creation_date: i32,
    pub name: String,
}

#[derive(PartialEq, PartialOrd, Insertable, Serialize, Deserialize, JsonSchema)]
#[diesel(table_name = crate::schema::Ingredient)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewIngredient {
    pub name: String,
}

#[derive(Queryable, Selectable, Insertable, Serialize, JsonSchema)]
#[diesel(table_name = crate::schema::Dish)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Dish {
    pub id: i32,
    pub creation_date: i32,
    pub prep_date: Option<i32>,
    pub name: Option<String>,
}

#[derive(Queryable, Selectable, Insertable, Serialize, Associations, JsonSchema)]
#[diesel(belongs_to(Dish))]
#[diesel(belongs_to(Ingredient))]
#[diesel(table_name = crate::schema::DishIngredient)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(primary_key(dish_id, ingredient_id))]
pub struct DishIngredient {
    pub creation_date: i32,
    pub dish_id: i32,
    pub ingredient_id: i32,
    pub weight: i32,
}

#[derive(Queryable, Selectable, Insertable, Serialize, JsonSchema)]
#[diesel(table_name = crate::schema::Dish)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewDish {
    pub name: Option<String>,
    pub prep_date: Option<i32>,
}

#[derive(Queryable, Selectable, Insertable, Serialize, JsonSchema)]
#[diesel(belongs_to(Dish))]
#[diesel(belongs_to(Ingredient))]
#[diesel(table_name = crate::schema::DishIngredient)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(primary_key(dish_id, ingredient_id))]
pub struct NewDishIngredient {
    pub dish_id: i32,
    pub ingredient_id: i32,
    pub weight: i32,
}

#[derive(Queryable, Selectable, Insertable, Serialize, Default, JsonSchema)]
#[diesel(table_name = crate::schema::Meal)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Meal {
    pub id: i32,
    pub creation_date: i32,
    pub eat_date: Option<i32>,
    pub duration: Option<i32>,
    pub description: Option<String>,
    pub hunger_level: Option<i32>,
    pub desire_to_eat: Option<i32>,
    pub fullness_afterwards: Option<i32>,
}

#[derive(Queryable, Selectable, Serialize, Default, JsonSchema)]
#[diesel(table_name = crate::schema::MealDish)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(primary_key(meal_id, dish_id))]
pub struct MealDish {
    pub creation_date: i32,
    pub dish_id: i32,
    pub meal_id: i32,
    pub weight: i32,
}

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize, Default, JsonSchema)]
#[diesel(table_name = crate::schema::Meal)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewMeal {
    pub eat_date: Option<i32>,
    pub duration: Option<i32>,
    pub description: Option<String>,
    pub hunger_level: Option<i32>,
    pub desire_to_eat: Option<i32>,
    pub fullness_afterwards: Option<i32>,
}

#[derive(Queryable, Selectable, Insertable, Serialize, Default, JsonSchema)]
#[diesel(table_name = crate::schema::MealDish)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(primary_key(meal_id, dish_id))]
pub struct NewMealDish {
    pub dish_id: i32,
    pub meal_id: i32,
    pub weight: i32,
}
