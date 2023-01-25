use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RequestUser {
    username: Option<String>,
    password: String
}