use rocket::{http::Status, serde::{json::Json, Deserialize, Serialize}};
use rocket_okapi::openapi;
use schemars::JsonSchema;
use chrono::{DateTime, FixedOffset, Utc};
use chrono_tz::Asia::Bangkok;

use crate::utils::input_validation;


#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
#[serde(crate = "rocket::serde")]
struct Flow {
    flow: f64,
}

#[derive(Deserialize, JsonSchema)]
#[serde(crate="rocket::serde")]
struct FlowRequestBody {
    #[schemars(schema_with = "crate::utils::schemars::datetime_schema")]
    time: String,
    weather_cond: String
}


#[openapi(tag = "Predictive")]
#[post("/flow", format = "json", data="<flow_request>")]
pub fn get_flow(flow_request: Json<FlowRequestBody>) -> Result<Json<Flow>, Status> {
    let request_data = flow_request.0;

    let (time, weather) = match input_validation::validate_time_weather(request_data.time, request_data.weather_cond) {
        Ok(r) => r,
        Err(e) => return Err(e)
    };
    
    println!("{:?}, {:?}", time, weather);

    Ok(rocket::serde::json::Json(Flow {
        flow: 50.1,
    }))
}