use crate::services::weather_forecast_service::WeatherForecastService;
use axum::response::IntoResponse;
use axum::Json;

pub async fn hello() -> String {
    "Hi RUST".to_owned()
}

pub async fn weather() -> impl IntoResponse {
    Json(WeatherForecastService::new().get_all())
}
