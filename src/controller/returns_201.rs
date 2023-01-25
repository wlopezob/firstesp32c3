use axum::response::{Response, IntoResponse};
use http::StatusCode;

pub async fn returns_201() -> Response {
    //(StatusCode::CREATED, "This is 201 created").into_response()
    (StatusCode::CREATED, ()).into_response()
}