use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestTask {
    pub title: String,
    pub priority: Option<String>,
    pub description: Option<String>
}