use axum::{http::StatusCode, Extension, Json};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, Set, IntoActiveModel,
};
use serde::{Deserialize, Serialize};

use crate::models::users;
use crate::models::users::Entity as Users;

#[derive(Deserialize)]
pub struct RequestUser {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct ResponseUser {
    username: String,
    id: i32,
    token: String,
}
pub async fn create_user(
    Extension(db): Extension<DatabaseConnection>,
    Json(request_user): Json<RequestUser>,
) -> Result<Json<ResponseUser>, StatusCode> {
    let new_user = users::ActiveModel {
        username: Set(request_user.username),
        password: Set(request_user.password),
        token: Set(Some("asdasdsadasd".to_owned())),
        ..Default::default()
    }
    .save(&db)
    .await
    .map_err(|err| {
        dbg!(err);
        return StatusCode::INTERNAL_SERVER_ERROR;
    })?;
    Ok(Json(ResponseUser {
        username: new_user.username.unwrap(),
        id: new_user.id.unwrap(),
        token: new_user.token.unwrap().unwrap(),
    }))
}

pub async fn login(
    Extension(db): Extension<DatabaseConnection>,
    Json(request_user): Json<RequestUser>,
) -> Result<Json<ResponseUser>, StatusCode> {
    let user = Users::find()
        .filter(
            Condition::all()
                .add(users::Column::Username.eq(request_user.username))
        )
        .one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(db_user) = user {
        let new_token = "123456789123456789".to_owned();
        let mut user = db_user.into_active_model();
        user.token = Set(Some(new_token));
        let saved_user = user.update(&db).await.map_err(|error| {
            dbg!(error);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
        Ok(Json(ResponseUser{
            username: saved_user.username,
            id: saved_user.id,
            token: saved_user.token.unwrap(),
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
