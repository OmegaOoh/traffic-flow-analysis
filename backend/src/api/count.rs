use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::http::Status;
use rocket_okapi::openapi;
use schemars::JsonSchema;
use crate::service::predictor::inference::{CarCountInferencer, HeavyVehicleCountInferencer, ModelInterface, MotorcycleCountInferencer, VehicleCountInferencer};
use crate::utils::input_validation;


#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Count {
    count: f64,
    vehicle_type: String,
}

#[derive(Deserialize, JsonSchema)]
#[serde(crate="rocket::serde")]
pub struct CountRequestBody {
    #[schemars(schema_with = "crate::utils::schemars::datetime_schema")]
    time: String,
    weather_cond: String,
}



#[openapi(tag = "Predictive")]
#[post("/count/<vehicle_type>", format = "json", data="<count_request>")]
pub fn get_count(vehicle_type: &str, count_request: Json<CountRequestBody> ) -> Result<Json<Count>, Status> {
    let request_data = count_request.0;

    
    let vtype: String = match input_validation::validate_vehicle(vehicle_type) {
        Ok(r) => r,
        Err(e) => return Err(e)
    };
    
    let (time, weather) = match input_validation::validate_time_weather(request_data.time, request_data.weather_cond) {
        Ok(r) => r,
        Err(e) => return Err(e)
    };
    
    let prediction_result: Result<f64, String> = match vtype.as_str() {
        "Motorcycle" => MotorcycleCountInferencer::inference(time, weather),
        "Car" => CarCountInferencer::inference(time, weather),
        "HeavyVehicle" => HeavyVehicleCountInferencer::inference(time, weather),
        _ => return Err(Status::InternalServerError),
    };
    
    
    let prediction: f64 = match prediction_result {
        Ok(r) => r,
        Err(_) => return Err(Status::InternalServerError),
    };

    Ok(Json(Count {
        count: prediction,
        vehicle_type: vtype,
    }))
}

#[openapi(tag = "Predictive")]
#[post("/count", format = "json", data="<count_request>")]
pub fn get_count_all(count_request: Json<CountRequestBody> ) -> Result<Json<Count>, Status> {
    let request_data = count_request.0;

    let (time, weather) = match input_validation::validate_time_weather(request_data.time, request_data.weather_cond) {
        Ok(r) => r,
        Err(e) => return Err(e)
    };
    
    let prediction: f64 = match VehicleCountInferencer::inference(time, weather) {
        Ok(r) => r,
        Err(_) => return Err(Status::InternalServerError),
    };

    Ok(Json(Count {
        count: prediction,
        vehicle_type: "Motorcycle, Car, HeavyVehicle".to_string(),
    }))
}

#[cfg(test)]
mod test {
    use rocket::{http::Status, serde::json::Json};
    use crate::api::count::{get_count, get_count_all, CountRequestBody};

    #[test]
    fn test_count_all_valid() {
        let request = Json(CountRequestBody {
            time: String::from("2023-05-15T14:30:00+07:00"),
            weather_cond: String::from("Clear"),
        });
        let response = get_count_all(request);
        assert!(response.is_ok());
        let response = response.unwrap().0;
        assert!(response.count >= 0.0);
        assert!(response.vehicle_type == "Motorcycle, Car, HeavyVehicle");
    }
    
    #[test]
    fn test_count_all_invalid_time() {
        let request = Json(CountRequestBody {
            time: String::from("a-05-15T14:30:00+07:00"),
            weather_cond: String::from("Clear"),
        });
        let response = get_count_all(request);
        assert!(response.is_err());
        assert_eq!(response.unwrap_err(), Status::BadRequest);
    }
    
    #[test]
    fn test_count_all_invalid_weather() {
        let request = Json(CountRequestBody {
            time: String::from("2023-05-15T14:30:00+07:00"),
            weather_cond: String::from("Candy"),
        });
        let response = get_count_all(request);
        assert!(response.is_err());
        assert_eq!(response.unwrap_err(), Status::BadRequest);
    }
    
    #[test]
    fn test_count_with_type_valid() {
        let request = Json(CountRequestBody {
            time: String::from("2023-05-15T14:30:00+07:00"),
            weather_cond: String::from("Clear"),
        });
        let response = get_count("car", request);
        assert!(response.is_ok());
        let response = response.unwrap().0;
        assert!(response.count >= 0.0);
        assert!(response.vehicle_type == "Car");
    }
    
    #[test]
    fn test_count_with_type_invalid() {
        let request = Json(CountRequestBody {
            time: String::from("2023-05-15T14:30:00+07:00"),
            weather_cond: String::from("Clear"),
        });
        let response = get_count("helicopter", request);
        assert!(response.is_err());
        assert_eq!(response.unwrap_err(), Status::NotFound);
    }
    
    
}