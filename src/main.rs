mod controller;
mod models;
mod services;
mod routes;
use crate::routes::init::run;

#[tokio::main]
async fn main() {
   run().await;
}