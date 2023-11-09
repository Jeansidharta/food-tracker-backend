
```usql database_sqlite
SELECT 
    dish.name as dish_name,
    ingredient.name as ingredient_name,
    dish.id as dish_id,
    ingredient.id as ingredient_id
FROM Dish
    JOIN dishIngredient ON Dish.id = dishIngredient.dish_id
    JOIN Ingredient on dishIngredient.ingredient_id = ingredient.id
WHERE dish.id = 13;
```

