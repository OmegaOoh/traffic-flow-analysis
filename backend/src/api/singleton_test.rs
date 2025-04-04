use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::openapi;
use schemars::JsonSchema;
use crate::service::predictor;
use std::sync::Mutex;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
#[serde(crate = "rocket::serde")]
struct Singleton {
    time: String,
}


// Returns a hello message
#[openapi(tag = "TEST")]
#[get("/singleton")]
pub fn singleton() -> rocket::serde::json::Json<Singleton> {
    let singleton: &'static Mutex<String> = predictor::get_data();
    let time_value = singleton.lock().unwrap().clone();
    rocket::serde::json::Json(Singleton { time: time_value })
}