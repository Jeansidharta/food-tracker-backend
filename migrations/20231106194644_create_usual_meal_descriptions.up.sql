CREATE TABLE UsualMealDescriptions (
	id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
	description TEXT NOT NULL
) STRICT;

INSERT INTO UsualMealDescriptions (id, description) VALUES
	(1, 'Dinner'),
	(2, 'Work Lunch'),
	(3, 'Lunch'),
	(4, 'Breakfast'),
	(5, 'Snack');
