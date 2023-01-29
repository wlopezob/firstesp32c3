use axum::http::StatusCode;
use axum::Json;
use dotenvy_macro::dotenv;
use serde::{Deserialize, Serialize};
use reqwest::{get, Error};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quotable {
    #[serde(rename(deserialize = "_id"))]
    pub id: String,
    pub tags: Vec<String>,
    pub content: String,
    pub author: String,
    #[serde(rename(deserialize = "authorSlug"))]
    pub author_slug: String,
    pub length: u64,
    #[serde(rename(deserialize = "dateAdded"))]
    pub date_added: String,
    #[serde(rename(deserialize = "dateModified"))]
    pub date_modified: String,
}

pub async fn get_random_quote() -> Result<Json<Quotable>, StatusCode> {
    let url  = dotenv!("API_URL_QUOTABLE");
    let response: Quotable = get(url)
        .await.map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?
        .json().await.map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(response))
}
