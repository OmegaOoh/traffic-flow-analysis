use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket_okapi::openapi;
use schemars::JsonSchema;
use chrono::{DateTime, FixedOffset, Utc};
use chrono_tz::Asia::Bangkok;


#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
#[serde(crate = "rocket::serde")]
struct Density {
    density: f32,
    weather_cond: String,
    time: String,
}

#[derive(Deserialize, JsonSchema)]
#[serde(crate="rocket::serde")]
struct DensityRequestBody {
    #[schemars(schema_with = "crate::utils::schemars::datetime_schema")]
    time: Option<String>,
    weather_cond: Option<String>
}


#[openapi(tag = "Predictive")]
#[post("/density", format = "json", data="<density_request>")]
pub fn get_density(density_request: Json<DensityRequestBody>) -> Json<Density> {
    let request_data = density_request.0;

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

    rocket::serde::json::Json(Density {
        density: 2.45,
        weather_cond: weather,
        time: time_str
    })
}