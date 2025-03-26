use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::openapi;
use schemars::JsonSchema;
use rocket_db_pools::sqlx;
use rocket_db_pools::sqlx::Row;

use crate::service::database_connection::Logs;

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
struct WeatherDataFull {
    date: Option<String>,
    time: Option<String>,
    temp: Option<i16>,
    weather: Option<String>,
    weather_desc: Option<String>,
    wind_speed: Option<f32>,
    wind_heading: Option<i16>,
    rainmmh: Option<f32>,
    humidity: Option<i16>
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(crate = "rocket::serde")]
struct WeatherShort {
    date: Option<String>,
    time: Option<String>,
    temp: Option<f32>,
    weather: Option<String>,
    humidity: Option<i8>,
}

#[openapi(tag="Raw Data")]
#[get("/weather", rank=2)]
pub fn get_all_weather_docs() -> rocket::serde::json::Json<Vec<WeatherShort>> {
    // Open API documentation
    rocket::serde::json::Json(vec![])
}


#[get("/weather")]
pub async fn get_all_weather(
    db: &Logs) -> rocket::serde::json::Json<Vec<WeatherShort>>
{
    let rows = sqlx::query(
        r#"
        SELECT 
            DATE_FORMAT(time, '%Y-%m-%d') as date,
            TIME_FORMAT(time, '%H:%i:%s') as time,
            temp,
            weather,
            humidity
        FROM weatherData
        LIMIT 50
        "#
    )
    .fetch_all(&db.0)
    .await
    .unwrap_or_default();

    // Map the results to struct
    let weather_data: Vec<WeatherShort> = rows.into_iter()
        .map(|row| WeatherShort {
            date: row.try_get("date").ok(),
            time: row.try_get("time").ok(),
            temp: row.try_get("temp").ok(),
            weather: row.try_get("weather").ok(),
            humidity: row.try_get::<i8, _>("humidity").ok(),
        })
        .collect();

    rocket::serde::json::Json(weather_data)
}