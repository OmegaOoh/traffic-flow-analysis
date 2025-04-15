use chrono::{DateTime, FixedOffset};
use rocket::http::Status;

use crate::service::predictor::pre_process;

pub fn validate_time_weather(time_input: String, weather: String) -> Result<(DateTime<FixedOffset>, String), Status> {
    let time = match time_input.parse::<DateTime<FixedOffset>>() {
        Ok(t) => t,
        Err(_) => return Err(Status::BadRequest),
    };

    if !pre_process::is_weather_valid(&(weather.as_str())) {
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

#[cfg(test)]
pub(crate) mod test_weather_validation {
    use super::*;
    
    #[test]
    fn test_validate_weather_valid() {
        let result = validate_time_weather(String::from("2024-04-25T12:00:00Z"),
                                           String::from("Clear"));
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_validate_weather_invalid_weather() {
        let result = validate_time_weather(String::from("2024-04-25T12:00:00Z"),
                                           String::from("Invalid"));
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), Status::BadRequest);
    }
    
    #[test]
    fn test_validate_weather_invalid_time() {
        let result = validate_time_weather(String::from("2024-04-25T25:00:00Z"),
                                           String::from("Clear"));
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), Status::BadRequest);
    }
    
}

#[cfg(test)]
mod test_vehicle_validation {
    use super::*;
    
    #[test]
    fn test_validate_vehicle_motorcycle() {
        let result = validate_vehicle("motorcycle");
        assert!(result.is_ok());
        assert!(result.unwrap() == "Motorcycle");
    }
    
    #[test]
    fn test_validate_vehicle_car() {
        let result = validate_vehicle("car");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Car");
    }
    
    #[test]
    fn test_validate_vehicle_heavyvehicle() {
        let result = validate_vehicle("heavyvehicle");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(),"HeavyVehicle");
    }
    
    #[test]
    fn test_validate_vehicle_invalid() {
        let result = validate_vehicle("invalid");
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(),Status::BadRequest);
    }
    
}