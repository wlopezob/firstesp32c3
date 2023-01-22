use chrono::{Duration, Utc};
use rand::{thread_rng, Rng};

use crate::models::weather_forecast::WeatherForecast;

pub trait WeatherForecastServiceInterface {
    fn get_all(&self) -> Vec<WeatherForecast>;
}

pub struct WeatherForecastService {}

impl WeatherForecastServiceInterface for WeatherForecastService {
    fn get_all(&self) -> Vec<WeatherForecast> {
        let summaries: [&str; 7] = [
            "Freezing", "Bracing", "Chilly", "Cool", "Mild", "Warm", "Balmy",
        ];
        let mut vec_weather = Vec::new();
        for n in summaries.into_iter().enumerate() {
            let weather = WeatherForecast::new(
                Utc::now() + Duration::days(n.0 as i64),
                thread_rng().gen_range(-20..55),
                summaries[thread_rng().gen_range(0..summaries.len())].to_string(),
            );
            vec_weather.push(weather);
        }
        vec_weather
    }
}

impl WeatherForecastService {
    pub fn new() -> Box<dyn WeatherForecastServiceInterface> {
        Box::new(WeatherForecastService {})
    }
}