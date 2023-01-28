use crate::{
    models::users::{self, Entity as Users},
    utils::{app_error::AppError, jwt::is_valid},
};
use axum::{
    headers::{authorization::Bearer, Authorization, HeaderMapExt},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response, extract::State, TypedHeader,
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub async fn guard<B>(
    State(db): State<DatabaseConnection>,
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, AppError> {
    let token = token.token().to_owned();
    // let token = req
    //     .headers()
    //     .typed_get::<Authorization<Bearer>>()
    //     .ok_or_else(|| AppError::new(StatusCode::BAD_REQUEST, "Missing bearer token"))?
    //     .token()
    //     .to_owned();
    let user = Users::find()
        .filter(users::Column::Token.eq(token.clone()))
        .one(&db)
        .await
        .map_err(|error| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, error.to_string()))?;
    is_valid(&token)?;
    let Some(user) = user else { return Err(AppError::new(StatusCode::UNAUTHORIZED, "You are not authorized, please log in")); };
    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}
