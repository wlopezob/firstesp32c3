use axum::{Extension, Json};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

use crate::models::{request_task::RequestTask, tasks};

pub async fn create_task(
    Extension(database): Extension<DatabaseConnection>,
    Json(request_task): Json<RequestTask>,
) {
    let new_task = tasks::ActiveModel {
        priority: Set(request_task.priority),
        title: Set(request_task.title),
        description: Set(request_task.description),
        is_default: Set(Some(true)),
        ..Default::default()
    };
    let result = new_task.save(&database).await.unwrap();
    dbg!(result);
}
