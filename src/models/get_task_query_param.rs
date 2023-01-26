use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetTaskQueryParam {
    pub priority : Option<String>
}