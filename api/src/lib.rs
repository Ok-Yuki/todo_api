pub mod db;
mod error_handler;
mod schema;
pub mod todo;

#[macro_use] 
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate serde_json;

