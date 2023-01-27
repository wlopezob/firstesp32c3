use crate::models::tasks::Entity as Tasks;
use crate::models::{atomic_update::RequestTaskUpdate, tasks};
use axum::{extract::Path, Extension, Json};
use http::StatusCode;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, Set};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestTaskUpdateOptional {
    pub id: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub priority: Option<Option<String>>,
    pub title: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub completed_at: Option<Option<DateTimeWithTimeZone>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub description: Option<Option<String>>,
    pub deleted_at: Option<Option<DateTimeWithTimeZone>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub user_id: Option<Option<i32>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub is_default: Option<Option<bool>>,
}

pub async fn update_task(
    Path(task_id): Path<i32>,
    Extension(db): Extension<DatabaseConnection>,
    Json(request_task): Json<RequestTaskUpdate>,
) -> Result<(), StatusCode> {
    let task_update = tasks::ActiveModel {
        id: Set(task_id),
        priority: Set(request_task.priority),
        title: Set(request_task.title),
        completed_at: Set(request_task.completed_at),
        description: Set(request_task.description),
        deleted_at: Set(request_task.deleted_at),
        user_id: Set(request_task.user_id),
        is_default: Set(request_task.is_default),
    };
    Tasks::update(task_update)
        .filter(tasks::Column::Id.eq(task_id))
        .exec(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(())
}

pub async fn partial_update(
    Path(task_id): Path<i32>,
    Extension(db): Extension<DatabaseConnection>,
    Json(request_task): Json<RequestTaskUpdateOptional>,
) -> Result<(), StatusCode> {
    let db_task = Tasks::find_by_id(task_id)
        .one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut db_task = if let Some(task) = db_task {
        task.into_active_model()
    } else {
        return Err(StatusCode::NOT_FOUND);
    };

    if let Some(priority) = request_task.priority {
        db_task.priority = Set(priority);
    }
    Tasks::update(db_task)
        .filter(tasks::Column::Id.eq(task_id))
        .exec(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(())
}
