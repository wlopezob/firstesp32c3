use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct RequestUser {
    #[validate(email(message = "must be a email valid"))]
    username: String,

    #[validate(length(min=8, message="must have at least 8 character"))]
    password: String
}