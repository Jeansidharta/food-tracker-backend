CREATE TABLE IngredientProperties (
	ingredient_id INTEGER PRIMARY KEY NOT NULL,
	product_code TEXT UNIQUE NOT NULL,
	open_food_facts_json JSON NOT NULL,

	kcal_100g INTEGER AS (json_extract(open_food_facts_json, '$.product.nutriments.energy-kcal_100g')) STORED,
	FOREIGN KEY (ingredient_id) REFERENCES Ingredient (id)
);
