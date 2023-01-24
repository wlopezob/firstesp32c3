use axum::{
    // headers::{authorization::Bearer, Authorization},
    middleware::Next,
    response::Response,
    // TypedHeader,
};
use http::{Request, StatusCode};

use crate::models::header_message::HeaderMessage;

pub async fn set_middleware_custom_header<B>(
    // TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let headers = request.headers();
    let message = headers
        .get("message")
        .ok_or(StatusCode::BAD_REQUEST)?;
    let message = message
        .to_str()
        .map_err(|_error| StatusCode::BAD_REQUEST)?
        .to_owned();
    let extension = request.extensions_mut();
    extension.insert(HeaderMessage(message));
    Ok(next.run(request).await)
}
