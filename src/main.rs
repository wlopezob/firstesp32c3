mod controller;
mod models;
mod services;
mod routes;
mod custom_middleware;
use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use crate::routes::init::run;

#[tokio::main]
async fn main() {
   // load environment variables from .env file
   dotenv().expect(".env file not found");
   let database_uri = dotenv!("DATABASE_URL");
   //database::run(database_uri);
   run(database_uri).await;
}