use axum::{Json, response::IntoResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MirrorJson {
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MirrorJsonResponse {
    message: String,
    message_from_serve: String,
}

pub async fn mirror_body_json(Json(body): Json<MirrorJson>) -> impl IntoResponse {
    dbg!(&body);
    Json(MirrorJsonResponse {
        message: body.message,
        message_from_serve: "Hello from RUST server".to_owned(),
    })
}
