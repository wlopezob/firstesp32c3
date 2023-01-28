use axum::extract::FromRef;


#[derive(Clone)]
pub struct SharedData {
    pub message: String,
}

impl SharedData {
    pub fn new(message: String) -> SharedData {
        SharedData { message }
    }
}
