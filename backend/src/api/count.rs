use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket_db_pools::sqlx::query;
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

#[derive(FromForm, JsonSchema)]
struct CountQuery {
    vehicle_type: Option<String>
}


#[openapi(tag = "Flow Analysis")]
#[post("/count?<query..>", format = "json", data="<count_request>")]
pub fn get_count(count_request: Json<CountRequestBody>, query: CountQuery ) -> Json<Count> {
    let request_data = count_request.0;

    let vehicle_type = match query.vehicle_type.as_deref() {
        Some("Motorcycle") => "Motorcycle",
        Some("Car") => "Car",
        Some("HeavyVehicle") => "HeavyVehicle",
        None => "Motorcycle, Car, HeavyVehicle",
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