use axum::{response::IntoResponse, Json, http::StatusCode};
use dotenvy_macro::dotenv;
use http::HeaderMap;
use reqwest::{get, Error};
use serde::{Serialize, Deserialize};

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
    // let client = reqwest::Client::new();
    // let mut headers = HeaderMap::new();
    let text = reqwest::get(url)
    .await.map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?
    .text()
    .await.map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;
    /*headers.insert(
        "Origin",
        "https://aplicaciones.mininter.gob.pe".parse().unwrap(),
    );
    headers.insert(
        "Host",
        "seguridadciudadana.mininter.gob.pe".parse().unwrap(),
    );
    headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36".parse().unwrap());
    headers.insert("Accept-Encoding", "gzip, deflate, br".parse().unwrap());
*/
    // let text = client
    //     .get(url)
    //     .headers(headers)
    //     .send()
    //     .await.map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?
    //     .text_with_charset("utf-8")
    //     .await.map_err(|error| {
    //         dbg!(&error);
    //         StatusCode::INTERNAL_SERVER_ERROR
    //     })?;
    Ok(text)
}
