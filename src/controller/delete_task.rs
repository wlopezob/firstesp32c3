use crate::models::tasks::{self, Entity as Tasks};
use axum::{
    extract::{Path, Query},
    Extension,
    http::StatusCode,
};
use sea_orm::{
    entity::ModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryParams {
    soft: bool,
}
pub async fn delete_task(
    Path(task_id): Path<i32>,
    Extension(db): Extension<DatabaseConnection>,
    Query(query_params): Query<QueryParams>,
) -> Result<(), StatusCode> {
    /*let task = Tasks::find_by_id(task_id).one(&db)
    .await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let task = if let Some(task) =  task {
        task.into_active_model()
    } else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };
    Tasks::delete(task).exec(&db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;*/

    // Tasks::delete_by_id(task_id)
    //     .exec(&db)
    //     .await
    //     .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if query_params.soft {
        let task = Tasks::find_by_id(task_id)
            .one(&db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let mut task = if let Some(task) = task {
            task.into_active_model()
        } else {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        };
        let now = chrono::Utc::now();
        task.deleted_at = Set(Some(now.into()));
        Tasks::update(task)
            .exec(&db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    } else {
        Tasks::delete_many()
            .filter(tasks::Column::Id.eq(task_id))
            .exec(&db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    Ok(())
}
