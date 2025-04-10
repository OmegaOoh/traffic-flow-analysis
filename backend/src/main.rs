#[macro_use]
extern crate rocket;

mod api;
mod service;
mod utils;

// Environment Variables
use dotenv::dotenv;
use std::env;

use rocket::http::Method;
use rocket_cors::{
    AllowedHeaders, AllowedOrigins, Error,
    Cors, CorsOptions 
};
use rocket_okapi::openapi_get_routes;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use service::database_connection::Logs;
use rocket_db_pools::Database;

fn make_cors() -> Cors {     
    let allowed_origins= AllowedOrigins::some_exact(&[
        env::var("ALLOWED_ORIGINS").unwrap_or("http://localhost:5173".to_string())
    ]);
    
    CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Put, Method::Delete].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "Content-Type"]),
        allow_credentials: true,
        ..Default::default()
    }.to_cors().expect("Failed to create CORS options")
}

#[launch]
fn rocket() -> _ {
    dotenv().ok(); // Load environment variable from .env file
    
    
    rocket::build()
        .attach(Logs::init())
        .mount("/api/v1", routes![
            api::weather_data::get_all_weather,
            api::vehicle_data::get_all_vehicle,
            api::flow_data::get_all_flow,
        ])
        .mount("/api/v1", openapi_get_routes![
            api::hello::hello_world,
            api::weather_data::get_all_weather_docs,
            api::vehicle_data::get_all_vehicle_docs,
            api::flow_data::get_all_flow_docs,
            api::flow::get_flow,
            api::density::get_density,
            api::count::get_count,
        ])
        .mount(
            "/api/v1/ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "/api/v1/openapi.json".to_string(),
                ..Default::default()
            }),
        )
        .attach(make_cors())
}
