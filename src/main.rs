use axum::{routing::get, Json, Router};
use chrono::{DateTime, Utc, Duration};
use rand::{thread_rng, Rng};
use serde::Serialize;

#[derive(Serialize)]
struct WeatherForecast {
    date: DateTime<Utc>,
    temperature: i32,
    summary: String,
}

impl WeatherForecast {
    fn new(date: DateTime<Utc>, temperature: i32, summary: String) -> WeatherForecast {
        WeatherForecast {
            date,
            temperature,
            summary,
        }
    }
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello world" }))
        .route("/hi", get(hi))
        .route("/weather", get(weather));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hi() -> String {
    "Hi RUST".to_owned()
}

async fn weather() -> Json<Vec<WeatherForecast>> {
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
    };
    Json(vec_weather)
}
