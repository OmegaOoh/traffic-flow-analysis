use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::openapi;
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub(crate) struct Alive {
    alive: bool,
}


#[openapi(tag = "Alive")]
#[get("/alive")]
pub fn is_alive() -> rocket::serde::json::Json<Alive> {
    rocket::serde::json::Json(Alive {
        alive: true
    })
}