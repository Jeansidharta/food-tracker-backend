watch:
	DATABASE_URL=sqlite:./db_dev PORT=8000 cargo watch -w src -x run

copy_prod:
	usql ~/.local/state/foodtracker.sqlite3 -c "PRAGMA wal_checkpoint(TRUNCATE)"
	cp ~/.local/state/foodtracker.sqlite3 ./db_dev.sqlite3
