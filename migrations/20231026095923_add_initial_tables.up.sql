CREATE TABLE Meal (
	id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
	creation_date INTEGER NOT NULL DEFAULT (unixepoch() * 1000),
	eat_date INTEGER,
	duration INTEGER,
	description TEXT,
	hunger_level INTEGER,
	desire_to_eat INTEGER,
	fullness_afterwards INT
) STRICT;

CREATE TABLE MealDish(
	creation_date INTEGER NOT NULL DEFAULT (unixepoch() * 1000),
	dish_id INTEGER REFERENCES Dish(id),
	meal_id INTEGER REFERENCES Meal(id),
	weight INT NOT NULL,
	PRIMARY KEY(dish_id, meal_id)
) STRICT;

CREATE TABLE Dish(
	id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
	creation_date INTEGER NOT NULL DEFAULT (unixepoch() * 1000),
	prep_date INTEGER,
	name TEXT
) STRICT;

CREATE TABLE DishIngredient (
	creation_date INTEGER NOT NULL DEFAULT (unixepoch() * 1000),
	dish_id INTEGER REFERENCES Dish(id),
	ingredient_id INTEGER REFERENCES Ingredient(id),
	weight INT NOT NULL,
	PRIMARY KEY(dish_id, ingredient_id)
) STRICT;

CREATE TABLE Ingredient(
	id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
	creation_date INTEGER NOT NULL DEFAULT (unixepoch() * 1000),
	name TEXT NOT NULL
) STRICT;

