use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::openapi;
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
#[serde(crate = "rocket::serde")]
struct Hello {
    msg: String,
    tag: Option<String>
}


// Returns a hello message
#[openapi(tag = "Hello")]
#[get("/hi")]
pub fn hello_world() -> rocket::serde::json::Json<Hello> {
    rocket::serde::json::Json(Hello {
        msg: "Welcome to Rocket 🚀🚀🚀".to_string(),
        tag: Some("Hello, World".to_string())
    })
}