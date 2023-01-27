use sea_orm::prelude::DateTimeWithTimeZone;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestTaskUpdate {
    pub id: Option<i32>,
    pub priority: Option<String>,
    pub title: String,
    pub completed_at: Option<DateTimeWithTimeZone>,
    pub description: Option<String>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
    pub user_id: Option<i32>,
    pub is_default: Option<bool>,
}