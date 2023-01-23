use crate::controller::{home_controller, json_controller, path_variable_controller};
use axum::{routing::{get, post}, Router};
use tower_http::cors::{Any, CorsLayer};

pub async fn run() {
    // build our application with a single route
    let app = create_routes();

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn create_routes() -> Router {
    let cors = CorsLayer::new().allow_origin(Any);
    // build our application with a single route
    Router::new()
        .layer(cors)
        .route("/", get(|| async { "Hello world" }))
        .route("/hello", get(home_controller::hello))
        .route("/mirror_body_string", post(home_controller::text))
        .route("/mirror_body_json", post(json_controller::mirror_body_json))
        .route("/path_variables/:id/variable/:subid", post(path_variable_controller::path_variables))
        .route("/weather", get(home_controller::weather))
}
