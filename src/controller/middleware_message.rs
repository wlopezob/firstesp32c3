use axum::extract::{State};

use crate::models::shared_data::SharedData;

pub async fn middleware_message(State(shared_data): State<SharedData>) -> String {
    shared_data.message
}