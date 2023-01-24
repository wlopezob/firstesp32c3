mod controller;
mod models;
mod services;
mod routes;
mod custom_middleware;
use crate::routes::init::run;

#[tokio::main]
async fn main() {
   run().await;
}