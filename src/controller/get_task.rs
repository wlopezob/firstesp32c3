use axum::{
    extract::{Path, Query},
    http::StatusCode,
    Extension, Json,
};
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};

use crate::models::{
    get_task_query_param::GetTaskQueryParam,
    response_task::ResponseTask,
    tasks::{self, Entity as Tasks},
};

pub async fn get_one_task(
    Extension(database): Extension<DatabaseConnection>,
    Path(task_id): Path<i32>,
) -> Result<Json<ResponseTask>, StatusCode> {
    let task = Tasks::find_by_id(task_id).one(&database).await.unwrap();
    if let Some(task) = task {
        Ok(Json(ResponseTask {
            id: task.id,
            title: task.title,
            priority: task.priority,
            description: task.description,
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn get_all_tasks(
    Extension(db): Extension<DatabaseConnection>,
    Query(query_params): Query<GetTaskQueryParam>,
) -> Result<Json<Vec<ResponseTask>>, StatusCode> {
    let mut priority_filter = Condition::all();
    if let Some(priority) = query_params.priority {
        priority_filter = if priority.is_empty() {
            priority_filter.add(tasks::Column::Priority.is_null())
        } else {
            priority_filter.add(tasks::Column::Priority.eq(priority))
        }
    }
    dbg!(&priority_filter);
    let tasks = Tasks::find()
        .filter(priority_filter)
        .all(&db)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|db_task| ResponseTask {
            id: db_task.id,
            title: db_task.title,
            priority: db_task.priority,
            description: db_task.description,
        })
        .collect();
    Ok(Json(tasks))
}
