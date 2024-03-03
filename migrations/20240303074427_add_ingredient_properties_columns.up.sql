ALTER TABLE IngredientProperties
ADD COLUMN product_name TEXT AS (json_extract(open_food_facts_json, '$.product.product_name')) VIRTUAL;

ALTER TABLE IngredientProperties
ADD COLUMN proteins_100g INTEGER AS (json_extract(open_food_facts_json, '$.product.nutriments.proteins_100g')) VIRTUAL;

ALTER TABLE IngredientProperties
ADD COLUMN fat_100g INTEGER AS (json_extract(open_food_facts_json, '$.product.nutriments.fat_100g')) VIRTUAL;

ALTER TABLE IngredientProperties
ADD COLUMN carbohydrates_100g INTEGER AS (json_extract(open_food_facts_json, '$.product.nutriments.carbohydrates_100g')) VIRTUAL;
