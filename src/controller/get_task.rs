use axum::{extract::Path, response::IntoResponse, Extension, Json};
use http::StatusCode;
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::models::{response_task::ResponseTask, tasks::Entity as Tasks};

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
) -> Result<Json<Vec<ResponseTask>>, StatusCode> {
    let tasks = Tasks::find()
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
