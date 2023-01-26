use axum::{extract::Path, response::IntoResponse, Extension, Json};
use http::StatusCode;
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::models::{tasks::Entity as Tasks, response_task::ResponseTask};

pub async fn get_task(
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
