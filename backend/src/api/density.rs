use rocket::{http::Status, serde::{json::Json, Deserialize, Serialize}};
use rocket_okapi::openapi;
use schemars::JsonSchema;

use crate::service::predictor::{DensityInferencer, ModelInterface};
use crate::utils::input_validation;


#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Density {
    density: f64,
}

#[derive(Deserialize, JsonSchema)]
#[serde(crate="rocket::serde")]
pub struct DensityRequestBody {
    #[schemars(schema_with = "crate::utils::schemars::datetime_schema")]
    time: String,
    weather_cond: String
}


#[openapi(tag = "Predictive")]
#[post("/density", format = "json", data="<density_request>")]
pub fn get_density(density_request: Json<DensityRequestBody>) -> Result<Json<Density>,Status> {
    let request_data = density_request.0;

    let (time, weather) = match input_validation::validate_time_weather(request_data.time, request_data.weather_cond) {
        Ok(r) => r,
        Err(e) => return Err(e)
    };
    
    let prediction: f64 = match DensityInferencer::inference(time, weather) {
        Ok(r) => r,
        Err(_) => return Err(Status::InternalServerError),
    };

    Ok(rocket::serde::json::Json(Density {
        density: prediction,
    }))
}