use axum::extract::Extension;

use crate::models::shared_data::SharedData;

pub async fn middleware_message(Extension(shared_data): Extension<SharedData>) -> String {
    shared_data.message
}