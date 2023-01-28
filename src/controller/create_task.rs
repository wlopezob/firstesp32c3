use axum::{
    headers::{
        authorization::{Bearer},
        Authorization,
    },
    Extension, Json, TypedHeader,
    http::StatusCode
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

use crate::models::users::Entity as Users;
use crate::models::{request_task::RequestTask, tasks, users};
pub async fn create_task(
    Extension(database): Extension<DatabaseConnection>,
    authorization: TypedHeader<Authorization<Bearer>>,
    Json(request_task): Json<RequestTask>,   
) -> Result<(), StatusCode> {
    let token = authorization.token();
    let user = Users::find()
        .filter(users::Column::Token.eq(token))
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let user = if let Some(db_user) = user {
        db_user
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let new_task = tasks::ActiveModel {
        user_id: Set(Some(user.id)),
        priority: Set(request_task.priority),
        title: Set(request_task.title),
        description: Set(request_task.description),
        is_default: Set(Some(true)),
        ..Default::default()
    };
    let result = new_task.save(&database).await.unwrap();
    dbg!(result);
    Ok(())
}
