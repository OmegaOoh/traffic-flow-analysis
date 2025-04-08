use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket_db_pools::sqlx::query;
use rocket::http::Status;
use rocket_okapi::openapi;
use schemars::JsonSchema;
use chrono::{DateTime, FixedOffset, Utc};
use chrono_tz::Asia::Bangkok;

use crate::service::predictor;


#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
#[serde(crate = "rocket::serde")]
struct Count {
    count: f64,
    vehicle_type: String,
    weather_cond: String,
    time: String,
}

#[derive(Deserialize, JsonSchema)]
#[serde(crate="rocket::serde")]
struct CountRequestBody {
    #[schemars(schema_with = "crate::utils::schemars::datetime_schema")]
    time: Option<String>,
    is_weekend: Option<bool>,
    weather_cond: Option<String>,
}

#[derive(FromForm, JsonSchema)]
struct CountQuery {
    vehicle_type: Option<String>
}


#[openapi(tag = "Flow Analysis")]
#[post("/count?<query..>", format = "json", data="<count_request>")]
pub fn get_count(count_request: Json<CountRequestBody>, query: CountQuery) -> Result<Json<Count>, Status> {
    let request_data = count_request.0;

    let vehicle_type = match query.vehicle_type.as_deref() {
        Some("Motorcycle") => "Motorcycle",
        Some("Car") => "Car",
        Some("HeavyVehicle") => "HeavyVehicle",
        None => "Motorcycle, Car, HeavyVehicle",
        _ => return Err(Status::BadRequest),
    }.to_string();

    // Convert UTC time to Bangkok time (GMT+7)
    let time_input = request_data.time.unwrap_or_else(|| Utc::now().to_rfc3339());
    let time = match time_input.parse::<DateTime<FixedOffset>>() {
        Ok(t) => t,
        Err(_) => return Err(Status::BadRequest),
    };
    let bangkok_time = time.with_timezone(&Bangkok);
    let time_str = bangkok_time.to_rfc3339(); 

    let weather = request_data.weather_cond.unwrap_or_else(|| String::from("Clear"));

    if !predictor::is_weather_valid(&weather) {
        return Err(Status::BadRequest);
    }

    let is_weekend = request_data.is_weekend.unwrap_or(false);

    let prediction: f64 = predictor::count_interference(time, weather.clone(), is_weekend);

    Ok(rocket::serde::json::Json(Count {
        count: prediction,
        vehicle_type,
        weather_cond: weather,
        time: time_str
    }))
}