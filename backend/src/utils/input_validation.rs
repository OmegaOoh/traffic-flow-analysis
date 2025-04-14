use chrono::{DateTime, FixedOffset, Utc};
use rocket::http::Status;

use crate::{api::weather_data, service::predictor};

pub fn validate_time_weather(time_input: String, weather: String) -> Result<(DateTime<FixedOffset>, String), Status> {
    // Convert UTC time to Bangkok time (GMT+7)
    let time = match time_input.parse::<DateTime<FixedOffset>>() {
        Ok(t) => t,
        Err(_) => return Err(Status::BadRequest),
    };

    if !predictor::is_weather_valid(&(weather.as_str())) {
        return Err(Status::BadRequest);
    }

    Ok((time, weather.to_string()))
}

pub fn validate_vehicle(vehicle_type: &str) -> Result<String, Status> {
    let vtype: String;
    if vehicle_type == "motorcycle" {
        vtype = "Motorcycle".to_string()
    } else if vehicle_type == "car" {
        vtype = "Car".to_string();
    } else if vehicle_type == "heavyvehicle" {
        vtype = "HeavyVehicle".to_string()
    } else {
        return Err(Status::BadRequest);
    }

    Ok(vtype)
}
