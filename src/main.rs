#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;
#[macro_use]
extern crate tr;

mod users;
mod schema;
mod middleware;
mod db;
mod app;
mod utils;
mod constants;
mod models;
use dotenv::dotenv;
use note_utils::MyError;

#[actix_rt::main]
async fn main() -> Result<(), MyError> {
    dotenv().ok();
    app::init().await?;
    Ok(())
}
