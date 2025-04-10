use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket_okapi::openapi;
use schemars::JsonSchema;
use chrono::{DateTime, FixedOffset, Utc};
use chrono_tz::Asia::Bangkok;


#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
#[serde(crate = "rocket::serde")]
struct Count {
    count: i8,
    vehicle_type: String,
    weather_cond: String,
    time: String,
}

#[derive(Deserialize, JsonSchema)]
#[serde(crate="rocket::serde")]
struct CountRequestBody {
    #[schemars(schema_with = "crate::utils::schemars::datetime_schema")]
    time: Option<String>,
    weather_cond: Option<String>
}



#[openapi(tag = "Predictive")]
#[post("/count/<veh_type>", format = "json", data="<count_request>")]
pub fn get_count(veh_type: &str, count_request: Json<CountRequestBody> ) -> Json<Count> {
    let request_data = count_request.0;

    let vehicle_type = match veh_type {
        "motorcycle" => "Motorcycle",
        "car" => "Car",
        "heavyvehicle" => "HeavyVehicle",
        _ => return Json(Count {
            count: 0,
            vehicle_type: "Invalid".to_string(),
            weather_cond: "".to_string(),
            time: "".to_string()
        }),
    }.to_string();

    // Convert UTC time to Bangkok time (GMT+7)
    let time_input = request_data.time.unwrap_or_else(|| Utc::now().to_rfc3339());
    let time = time_input.parse::<DateTime<FixedOffset>>().unwrap_or_else(|_| {
        DateTime::parse_from_rfc3339(&Utc::now().to_rfc3339()).unwrap()
    });
    let bangkok_time = time.with_timezone(&Bangkok);
    let time_str = bangkok_time.to_rfc3339(); 
    //let time_str = bangkok_time..format("%Y-%m-%d %H:%M:%S %Z").to_string();

    let weather = request_data.weather_cond.unwrap_or_else(|| String::from("few clouds"));
    // TODO Add check for valid weather condition 


    rocket::serde::json::Json(Count {
        count: 10,
        vehicle_type,
        weather_cond: weather,
        time: time_str
    })
}

#[openapi(tag = "Predictive")]
#[post("/count", format = "json", data="<count_request>")]
pub fn get_count_all(count_request: Json<CountRequestBody> ) -> Json<Count> {
    let request_data = count_request.0;

    // Convert UTC time to Bangkok time (GMT+7)
    let time_input = request_data.time.unwrap_or_else(|| Utc::now().to_rfc3339());
    let time = time_input.parse::<DateTime<FixedOffset>>().unwrap_or_else(|_| {
        DateTime::parse_from_rfc3339(&Utc::now().to_rfc3339()).unwrap()
    });
    let bangkok_time = time.with_timezone(&Bangkok);
    let time_str = bangkok_time.to_rfc3339(); 
    //let time_str = bangkok_time..format("%Y-%m-%d %H:%M:%S %Z").to_string();

    let weather = request_data.weather_cond.unwrap_or_else(|| String::from("few clouds"));
    // TODO Add check for valid weather condition 


    rocket::serde::json::Json(Count {
        count: 30,
        vehicle_type: "Motorcycle, Car, HeavyVehicle".to_string(),
        weather_cond: weather,
        time: time_str
    })
}