use axum::extract::State;
use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::TypedHeader;
use axum::{http::StatusCode, Extension, Json};
use sea_orm::sea_query::{Expr, Value};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, IntoActiveModel,
    QueryFilter, Set,
};
use serde::{Deserialize, Serialize};

use crate::models::users::Entity as Users;
use crate::models::users::{self, Model};
use crate::utils::jwt::create_jwt;

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
    State(db): State<DatabaseConnection>,
    Json(request_user): Json<RequestUser>,
) -> Result<Json<ResponseUser>, StatusCode> {
    let jwt = create_jwt()?;
    let new_user = users::ActiveModel {
        username: Set(request_user.username),
        password: Set(hash_password(request_user.password)?),
        token: Set(Some(jwt)),
        ..Default::default()
    }
    .save(&db)
    .await
    .map_err(|err| {
        dbg!(err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(Json(ResponseUser {
        username: new_user.username.unwrap(),
        id: new_user.id.unwrap(),
        token: new_user.token.unwrap().unwrap(),
    }))
}

pub async fn login(
    State(db): State<DatabaseConnection>,
    Json(request_user): Json<RequestUser>,
) -> Result<Json<ResponseUser>, StatusCode> {
    let jwt = create_jwt()?;
    let user = Users::find()
        .filter(Condition::all().add(users::Column::Username.eq(request_user.username)))
        .one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(db_user) = user {
        if !verify_passwor(request_user.password,&db_user.password)? {
            return Err(StatusCode::UNAUTHORIZED);
        }
        let new_token = jwt;
        let mut user = db_user.into_active_model();
        user.token = Set(Some(new_token));
        let saved_user = user.update(&db).await.map_err(|error| {
            dbg!(error);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
        Ok(Json(ResponseUser {
            username: saved_user.username,
            id: saved_user.id,
            token: saved_user.token.unwrap(),
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

fn hash_password(password: String) -> Result<String, StatusCode> {
    bcrypt::hash(password, 14).map_err(|error| {
        dbg!(error);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

fn verify_passwor(password: String, hash_password: &str) -> Result<bool, StatusCode> {
    bcrypt::verify(password, hash_password).map_err(|error| {
        dbg!(error);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

pub async fn logout(
    //autorization: TypedHeader<Authorization<Bearer>>,
    Extension(user): Extension<Model>,
    State(db): State<DatabaseConnection>,
) -> Result<(), StatusCode> {
    let mut user = user.into_active_model();
    user.token = Set(None);
    user.update(&db).await.map_err(|error| {
        dbg!(error);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(())
    // let token = autorization.token();
    // let db_user = Users::update_many()
    //     .col_expr(users::Column::Token,  Expr::value(Value::String(None)))
    //     .filter(users::Column::Token.eq(token))
    //     .exec(&db)
    //     .await
    //     .map_err(|error| {
    //         dbg!(&error);
    //         StatusCode::INTERNAL_SERVER_ERROR
    //     })?;
    // if db_user.rows_affected > 0 {
    //     Ok(())
    // } else {
    //     Err(StatusCode::UNAUTHORIZED)
    // }
}
