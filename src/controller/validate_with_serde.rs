use axum::Json;

use crate::models::request_user::RequestUser;

pub async fn validate_with_serde(Json(user): Json<RequestUser>) {
    dbg!(&user);
}