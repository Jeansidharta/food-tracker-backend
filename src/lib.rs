#![allow(non_snake_case)]

mod app_error;
mod dish;
mod ingredient;
mod meal;
mod models;
mod server;

use schemars::JsonSchema;
use serde::Deserialize;
pub use server::server;
mod state;

pub fn get_missing_items(
    list: Vec<i64>,
    required_items: impl IntoIterator<Item = i64>,
) -> Vec<i64> {
    let mut unknown_ingredients = vec![];
    required_items.into_iter().for_each(|item| {
        if !list.contains(&item) {
            unknown_ingredients.push(item)
        }
    });
    unknown_ingredients
}

#[derive(JsonSchema, Deserialize)]
pub struct PathId {
    pub id: i64,
}
