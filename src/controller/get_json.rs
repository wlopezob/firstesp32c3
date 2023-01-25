use axum::Json;
use chrono::{Utc};

use crate::models::weather_forecast::WeatherForecast;

pub async fn get_json() -> Json<WeatherForecast> {
    let data = WeatherForecast::new(Utc::now(), 50, "very cool".to_string());
    Json(data)
}