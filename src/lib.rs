#![allow(non_snake_case)]

mod app_error;
mod dish;
mod ingredient;
mod meal;
mod models;
mod schema;
mod server;

pub use server::server;

pub fn establish_connection() -> diesel::SqliteConnection {
    use diesel::prelude::*;
    use std::env;

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    diesel::SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
