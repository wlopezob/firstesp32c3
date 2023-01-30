use axum::{http::StatusCode, response::IntoResponse, Json};
use dotenvy_macro::dotenv;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, HOST, AUTHORIZATION};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Departamento {
    pub display_field_name: Option<String>,
    pub field_aliases: Option<FieldAliases>,
    pub geometry_type: Option<String>,
    pub spatial_reference: Option<SpatialReference>,
    pub fields: Option<Vec<Field>>,
    pub features: Option<Vec<Feature>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldAliases {
    pub objectid: String,
    #[serde(rename = "id_dpto")]
    pub id_dpto: String,
    pub departamento: String,
    pub capital: String,
    pub fuente: String,
    #[serde(rename = "st_area(shape)")]
    pub st_area_shape: String,
    #[serde(rename = "st_length(shape)")]
    pub st_length_shape: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpatialReference {
    pub wkid: i64,
    pub latest_wkid: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub alias: String,
    pub length: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Feature {
    pub attributes: Attributes,
    pub geometry: Geometry,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub objectid: i64,
    #[serde(rename = "id_dpto")]
    pub id_dpto: String,
    pub departamento: String,
    pub capital: String,
    pub fuente: String,
    #[serde(rename = "st_area(shape)")]
    pub st_area_shape: f64,
    #[serde(rename = "st_length(shape)")]
    pub st_length_shape: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Geometry {
    pub rings: Vec<Vec<Vec<i64>>>,
}
pub async fn comisarias() -> Result<String, StatusCode> {
    let url = dotenv!("API_URL_DPTO");
    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();
    // headers.insert("Accept", HeaderValue::from_static("*/*"));
    // headers.insert("Cache-Control", HeaderValue::from_static("no-cache"));
    headers.insert(
        HOST,
        HeaderValue::from_static("seguridadciudadana.mininter.gob.pe"),
    );
    headers.insert("User-Agent", HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36"));
    headers.insert(
        "Accept-Encoding",
        HeaderValue::from_static("gzip, deflate, br"),
    );
    headers.insert("Connection", HeaderValue::from_static("keep-alive"));
    dbg!(&headers);
    let builder = reqwest::Client::builder()
        //.default_headers(headers)
        .build().map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;
    let res = builder
        .get(url)
        //.header(AUTHORIZATION, "Bearer [AUTH_TOKEN]")
        .headers(headers)
        .send()
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;

    //println!("Headers Host:\n{:#?}", res.headers().get("Host").unwrap());
    eprintln!("Response: {:?} {}", res.version(), res.status());
    eprintln!("Headers: {:#?}\n", res.headers());

    let body = res
        .text()
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;

    println!("{}", body);

    Ok(body)
}
pub async fn comisarias1() -> Result<String, StatusCode> {
    let url = dotenv!("API_URL_DPTO");
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    // let text = reqwest::get(url)
    // .await.map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?
    // .text()
    // .await.map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;
    headers.insert("Accept", HeaderValue::from_static("*/*"));
    headers.insert("Cache-Control", HeaderValue::from_static("no-cache"));
    headers.insert(
        "Host",
        HeaderValue::from_static("seguridadciudadana.mininter.gob.pe"),
    );
    headers.insert("User-Agent", HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36"));
    headers.insert(
        "Accept-Encoding",
        HeaderValue::from_static("gzip, deflate, br"),
    );
    headers.insert("Connection", HeaderValue::from_static("keep-alive"));
    let text = client
        .get(url)
        .headers(headers)
        .send()
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?
        .text_with_charset("utf-8")
        .await
        .map_err(|error| {
            dbg!(&error);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(text)
}
