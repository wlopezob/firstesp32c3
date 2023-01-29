use crate::{
    controller::{
        create_task::create_task,
        custom_json_extractor,
        delete_task::delete_task,
        get_json::get_json,
        get_task::{get_all_tasks, get_one_task},
        guard::guard,
        home_controller, json_controller, middleware_message, mirror_custom_header,
        mirror_user_agent, path_variable_controller, query_params, read_middleware_custom_header,
        returns_201, update_task,
        users::{create_user, login, logout},
        validate_with_serde, quotable::get_random_quote, comisaria::{comisarias},
    },
    custom_middleware::set_middleware_custom_header,
    models::shared_data::SharedData,
};
use axum::{
    extract::FromRef,
    middleware,
    routing::{delete, get, patch, post, put},
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

#[derive(Clone, FromRef)]
pub struct AppState {
    pub database: DatabaseConnection,
    pub shared_data: SharedData
}
fn create_routes(database: DatabaseConnection) -> Router {
    let cors = CorsLayer::new().allow_origin(Any);
    let shared_data = SharedData::new("hello from shared data, I am state now".to_owned());
    let app_state = AppState { database , shared_data: shared_data.clone()};
    // build our application with a single route
    Router::new()
        .route("/users/logout", post(logout)) //no aplica el guard
        .route("/tasks", post(create_task))
        .route("/tasks", get(get_all_tasks))
        .route("/tasks/:task_id", get(get_one_task))
        .route("/tasks/:task_id", put(update_task::update_task))
        .route("/tasks/:task_id", patch(update_task::partial_update))
        .route("/tasks/:task_id", delete(delete_task))
        .route_layer(middleware::from_fn_with_state(app_state.clone(), guard))
        .route("/", get(|| async { "Hello world" }))
        .route("/hello", get(home_controller::hello))
        // .route(
        //     "/read_middleware_custom_header",
        //     get(read_middleware_custom_header::read_middleware_custom_header),
        // )
        // .route_layer(middleware::from_fn(
        //     set_middleware_custom_header::set_middleware_custom_header,
        // ))
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
        .route("/users/login", post(login))
        .route("/users", post(create_user))
        .route("/quotable/random", get(get_random_quote))
        .route("/comisarias", get(comisarias))
        .layer(cors)
        .with_state(app_state)
}
