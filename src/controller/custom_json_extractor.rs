use axum::{async_trait, body::HttpBody, extract::FromRequest, BoxError, Json, RequestExt};
use http::{Request, StatusCode};
use validator::Validate;

use crate::models::request_user::RequestUser;

#[async_trait]
impl<S, B> FromRequest<S, B> for RequestUser
where
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request(request: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(user) = request
            .extract::<Json<RequestUser>, _>()
            .await
            .map_err(|error| (StatusCode::BAD_REQUEST, format!("{}", error)))?;
        if let Err(errors) = user.validate() {
            return Err((StatusCode::BAD_REQUEST, format!("{}", errors)));
        }
        Ok(user)
    }
}

pub async fn custom_json_extractor(user: RequestUser) {
    dbg!(user);
}
