use rocket::{http::Status, serde::{json::Json, Deserialize, Serialize}};
use rocket_okapi::openapi;
use schemars::JsonSchema;

use crate::service::predictor::inference::{FlowInferencer, ModelInterface};
use crate::utils::input_validation;


#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Flow {
    flow: f64,
}

#[derive(Deserialize, JsonSchema)]
#[serde(crate="rocket::serde")]
pub struct FlowRequestBody {
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
    
    let prediction = match FlowInferencer::inference(time, weather) {
        Ok(p) => p,
        Err(_) => return Err(Status::InternalServerError)
    };

    Ok(rocket::serde::json::Json(Flow {
        flow: prediction,
    }))
}

#[cfg(test)]
mod test {
    use rocket::{http::Status, serde::json::Json};

    use crate::api::flow::{get_flow, FlowRequestBody};

    #[test]
    fn test_flow_endpoint_all_valid() {
        let request = Json(FlowRequestBody {
            time: String::from("2023-05-15T14:30:00+07:00"),
            weather_cond: String::from("Clear"),
        });
        let response = get_flow(request);
        
        assert!(response.is_ok());
        let result = response.unwrap();
        assert!(result.flow > 0.0);
    }
    
    #[test]
    fn test_flow_endpoint_all_invalid_date() {
        let request = Json(FlowRequestBody {
            time: String::from("a-05-15T14:30:00+07:00"),
            weather_cond: String::from("Clear"),
        });
        let response = get_flow(request);
        
        assert!(response.is_err());
        assert_eq!(response.unwrap_err(), Status::BadRequest);
    }
    
    #[test]
    fn test_flow_endpoint_all_invalid_weather() {
        let request = Json(FlowRequestBody {
            time: String::from("2023-05-15T14:30:00+07:00"),
            weather_cond: String::from("Candy"),
        });
        let response = get_flow(request);
        
        assert!(response.is_err());
        assert_eq!(response.unwrap_err(), Status::BadRequest);
    }
}
