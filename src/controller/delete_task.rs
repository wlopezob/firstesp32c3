use crate::models::tasks::{Entity as Tasks, self};
use axum::{extract::Path, Extension};
use http::StatusCode;
use sea_orm::{entity::ModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, ColumnTrait};

pub async fn delete_task(
    Path(task_id): Path<i32>,
    Extension(db): Extension<DatabaseConnection>,
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

    Tasks::delete_many()
    .filter(tasks::Column::Id.eq(task_id))
    .exec(&db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(())
}
