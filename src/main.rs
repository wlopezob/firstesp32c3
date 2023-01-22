use axum::{routing::get, Router};
use tower_http::cors::{Any, CorsLayer};
mod models;
mod services;
mod controller;
#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_origin(Any);
    // build our application with a single route
    let app = Router::new()
        .layer(cors)
        .route("/", get(|| async { "Hello world" }))
        .route("/hi", get(controller::home_controller::hello))
        .route("/weather", get(controller::home_controller::weather));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}