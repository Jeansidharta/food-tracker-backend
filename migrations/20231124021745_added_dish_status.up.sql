ALTER TABLE Dish ADD COLUMN is_finished INTEGER NOT NULL DEFAULT 0;

UPDATE Dish
SET is_finished = CASE WHEN Remaining.remaining <= 0 THEN TRUE ELSE FALSE END
FROM (
    SELECT
        id,
        (CASE
            WHEN total_weight > 0 THEN total_weight
            ELSE ingredients_weight
        END - used_weight) as remaining
    FROM (
        SELECT
            id,
            total_weight,
            name,
            (
                SELECT TOTAL(MealDish.weight) as weight
                FROM MealDish WHERE dish_id = Dish.id
            ) as used_weight,
            (
                SELECT TOTAL(DishIngredient.weight) as weight
                FROM DishIngredient WHERE dish_id = Dish.id
            ) as ingredients_weight
        FROM Dish
    )
) as Remaining
WHERE Dish.id = Remaining.id;


