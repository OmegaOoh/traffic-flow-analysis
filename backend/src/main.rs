#[macro_use]
extern crate rocket;

mod api;
mod service;
mod utils;

use dotenv::dotenv;
use rocket_okapi::openapi_get_routes;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use service::database_connection::Logs;
use rocket_db_pools::Database;

#[launch]
fn rocket() -> _ {
    dotenv().ok(); // Load environment variable from .env file
    
    
    rocket::build()
        .attach(Logs::init())
        .mount("/api/v1", routes![
            api::weather_data::get_all_weather,
        ])
        .mount("/api/v1", openapi_get_routes![
            api::hello::hello_world,
            api::weather_data::get_all_weather_docs,
            api::flow::get_flow,
        ])
        .mount(
            "/api/v1/ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "/api/v1/openapi.json".to_string(),
                ..Default::default()
            }),
        )
}
