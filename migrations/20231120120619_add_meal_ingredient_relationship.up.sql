CREATE TABLE MealIngredient (
	creation_date INTEGER NOT NULL DEFAULT (unixepoch() * 1000),
	ingredient_id INTEGER REFERENCES Ingredient(id),
	meal_id INTEGER REFERENCES Meal(id),
	weight INTEGER NOT NULL,
	PRIMARY KEY(ingredient_id, meal_id)
) STRICT;
