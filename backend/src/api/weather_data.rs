use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::openapi;
use rocket_okapi::okapi::schemars;
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
    // Date of the weather record (YYYY-MM-DD)
    date: Option<String>,
    // Time of the weather record (HH:MM)
    time: Option<String>,
    // Temperature in Kelvin
    temp: Option<f32>,
    // Weather condition
    weather: Option<String>,
    // Humidity percentage
    humidity: Option<i8>,
}

#[derive(FromForm, JsonSchema)]
struct WeatherQuery {
    // Maximum number of record returns
    limit: Option<usize>,
    // Date of record to find (YYYY-MM-DD)
    date: Option<String>,
    // Time of record to find (HH:MM)
    time: Option<String>,
}

#[openapi(tag="Raw Data")]
#[get("/weather?<query..>", rank=2)]
pub fn get_all_weather_docs(query: WeatherQuery) -> rocket::serde::json::Json<Vec<WeatherShort>> {
    // Open API documentation
    rocket::serde::json::Json(vec![])
}


#[get("/weather?<query..>")]
pub async fn get_all_weather(
    query: WeatherQuery,
    db: &Logs) -> rocket::serde::json::Json<Vec<WeatherShort>>
{
    let limit = query.limit.unwrap_or(50);
    
    let mut  sql = String::from (
        "SELECT 
             DATE_FORMAT(time, '%Y-%m-%d') as date,
             TIME_FORMAT(time, '%H:%i') as time,
             temp,
             weather,
             humidity
         FROM weatherData"
    );
    
    if let Some(date) = &query.date {
        sql.push_str(&format!(" WHERE DATE(time) = '{}'", date))
    }
    
    if let Some(time) = &query.time {
        sql.push_str(&format!(" WHERE TIME_FORMAT(time, '%H:%i') = {}", time))
    }
    
    sql.push_str(&format!(" LIMIT {}", limit));
    
    let rows = sqlx::query(&sql)
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