use crate::{
    controller::{
        create_task::create_task, custom_json_extractor, get_json::get_json, home_controller,
        json_controller, middleware_message, mirror_custom_header, mirror_user_agent,
        path_variable_controller, query_params, read_middleware_custom_header, returns_201,
        validate_with_serde, get_task::get_task,
    },
    custom_middleware::set_middleware_custom_header,
    models::shared_data::SharedData,
};
use axum::{
    middleware,
    routing::{get, post},
    Extension, Router,
};
use sea_orm::{Database, DatabaseConnection};
use tower_http::cors::{Any, CorsLayer};

pub async fn run(database_uri: &str) {
    let database = Database::connect(database_uri).await.unwrap();
    // build our application with a single route
    let app = create_routes(database);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn create_routes(database: DatabaseConnection) -> Router {
    let cors = CorsLayer::new().allow_origin(Any);
    let shared_data = SharedData::new("hello from shared data".to_owned());

    // build our application with a single route
    Router::new()
        .route(
            "/read_middleware_custom_header",
            get(read_middleware_custom_header::read_middleware_custom_header),
        )
        .route_layer(middleware::from_fn(
            set_middleware_custom_header::set_middleware_custom_header,
        ))
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
        .route("/returns_201", post(returns_201::returns_201))
        .route("/weather", get(home_controller::weather))
        .route("/get_json", get(get_json))
        .route(
            "/validate_data",
            post(validate_with_serde::validate_with_serde),
        )
        .route(
            "/custom_json_extractor",
            post(custom_json_extractor::custom_json_extractor),
        )
        .route("/tasks", post(create_task))
        .route("/task/:task_id", get(get_task))
        .layer(cors)
        .layer(Extension(shared_data))
        .layer(Extension(database))
}
