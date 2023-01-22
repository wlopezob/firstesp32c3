use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct WeatherForecast {
    date: DateTime<Utc>,
    temperature: i32,
    summary: String,
}

impl WeatherForecast {
    pub fn new(date: DateTime<Utc>, temperature: i32, summary: String) -> WeatherForecast {
        WeatherForecast {
            date,
            temperature,
            summary,
        }
    }
}