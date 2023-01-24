use crate::{
    controller::{
        home_controller, json_controller, middleware_message, mirror_custom_header,
        mirror_user_agent, path_variable_controller, query_params,
    },
    models::shared_data::SharedData,
};
use axum::{
    routing::{get, post},
    Extension, Router,
};
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
    let shared_data = SharedData::new("hello from shared data".to_owned());

    // build our application with a single route
    Router::new()
        .route("/", get(|| async { "Hello world" }))
        .route("/hello", get(home_controller::hello))
        .route("/mirror_body_string", post(home_controller::text))
        .route("/mirror_body_json", post(json_controller::mirror_body_json))
        .route(
            "/path_variables/:id/variable/:subid",
            post(path_variable_controller::path_variables),
        )
        .route("/query_params", get(query_params::query_params))
        .route(
            "/mirror_user_agent",
            get(mirror_user_agent::mirror_user_agent),
        )
        .route(
            "/mirror_custom_header",
            get(mirror_custom_header::mirror_custom_header),
        )
        .route(
            "/middleware_message",
            get(middleware_message::middleware_message),
        )
        .route("/weather", get(home_controller::weather))
        .layer(cors)
        .layer(Extension(shared_data))
}
