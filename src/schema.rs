// @generated automatically by Diesel CLI.

diesel::table! {
    Dish (id) {
        id -> Integer,
        creation_date -> Integer,
        prep_date -> Nullable<Integer>,
        name -> Nullable<Text>,
    }
}

diesel::table! {
    DishIngredient (dish_id, ingredient_id) {
        creation_date -> Integer,
        dish_id -> Integer,
        ingredient_id -> Integer,
        weight -> Integer,
    }
}

diesel::table! {
    Ingredient (id) {
        id -> Integer,
        creation_date -> Integer,
        name -> Text,
    }
}

diesel::table! {
    Meal (id) {
        id -> Integer,
        creation_date -> Integer,
        eat_date -> Nullable<Integer>,
        duration -> Nullable<Integer>,
        description -> Nullable<Text>,
        hunger_level -> Nullable<Integer>,
        desire_to_eat -> Nullable<Integer>,
        fullness_afterwards -> Nullable<Integer>,
    }
}

diesel::table! {
    MealDish (dish_id, meal_id) {
        creation_date -> Integer,
        dish_id -> Integer,
        meal_id -> Integer,
        weight -> Integer,
    }
}

diesel::joinable!(DishIngredient -> Dish (dish_id));
diesel::joinable!(DishIngredient -> Ingredient (ingredient_id));
diesel::joinable!(MealDish -> Dish (dish_id));
diesel::joinable!(MealDish -> Meal (meal_id));

diesel::allow_tables_to_appear_in_same_query!(
    Dish,
    DishIngredient,
    Ingredient,
    Meal,
    MealDish,
);
