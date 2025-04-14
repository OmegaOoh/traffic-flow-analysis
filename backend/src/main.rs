#[macro_use]
extern crate rocket;

mod api;
mod service;
mod utils;

// Environment Variables
use dotenv::dotenv;
use std::env;

use rocket_okapi::openapi_get_routes;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use service::database_connection::Logs;
use service::predictor;
use rocket_db_pools::Database;
use rocket::http::Method;
use rocket_cors::{
    AllowedHeaders, AllowedOrigins,
    Cors, CorsOptions 
};

fn make_cors() -> Cors {    
   let origins_str :String  =  env::var("ALLOWED_ORIGINS").unwrap_or("http://localhost:8000".to_string());
   let origins = origins_str.as_str().split(",").collect::<Vec<&str>>();
    let allowed_origins= AllowedOrigins::some_exact(&origins);
    
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
    
    predictor::init_model();
    
    rocket::build()
    .attach(Logs::init()).attach(make_cors())
    .mount("/api/v2", routes![
        api::weather_data::get_all_weather,
        api::vehicle_data::get_all_vehicle,
        api::flow_data::get_all_flow,
    ])
    .mount("/api/v2", openapi_get_routes![
        api::weather_data::get_all_weather_docs,
        api::vehicle_data::get_all_vehicle_docs,
        api::flow_data::get_all_flow_docs,
        api::flow::get_flow,
        api::count::get_count,
        api::count::get_count_all,
    ])
    .mount(
        "/api/v2/ui/",
        make_swagger_ui(&SwaggerUIConfig {
            url: "/api/v2/openapi.json".to_string(),
            ..Default::default()
        }),
    )
}
