use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket_okapi::openapi;
use rocket_okapi::okapi::schemars;
use schemars::JsonSchema;
use rocket_db_pools::sqlx;
use rocket_db_pools::sqlx::Row;

use crate::service::database_connection::Logs;

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(crate = "rocket::serde")]
pub struct WeatherDesc {
    weather: Option<String>,
    count: i64
}

#[openapi(tag="Descriptive")]
#[get("/desc/weather")]
pub async fn get_all_weather(
    db: &Logs) -> Json<Vec<WeatherDesc>>
{   
    let sql = String::from (
        "SELECT
            CASE
                WHEN WeatherDesc IN ('clear sky', 'few clouds', 'scattered clouds') THEN 'Clear'
                WHEN WeatherDesc IN ('broken clouds', 'overcast clouds') THEN 'Cloudy'
                WHEN WeatherDesc IN ('haze', 'mist') THEN 'Low Visibility'
                WHEN WeatherDesc IN ('light rain', 'moderate rain', 'heavy intensity rain') THEN 'Rain'
                ELSE 'Other'
            END 
            AS WeatherGroup,
            COUNT(*) AS Count
        FROM weatherData
        GROUP BY WeatherGroup;
         "
    );
    
    let rows = sqlx::query(&sql)
    .fetch_all(&db.0)
    .await
    .unwrap_or_default();

    // Map the results to struct
    let weathers_counts: Vec<WeatherDesc> = rows.into_iter()
        .map(|row| WeatherDesc {
            weather: row.try_get("WeatherGroup").ok(),
            count: row.try_get("Count").unwrap_or_default()
        })
        .collect();

    Json(weathers_counts)
}