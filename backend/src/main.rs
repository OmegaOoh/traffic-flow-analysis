#[macro_use]
extern crate rocket;

mod api;

use rocket_okapi::openapi_get_routes;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

#[launch]
fn rocket() -> _ {
    
    rocket::build()
        .mount("/api/v1", openapi_get_routes![api::hello::hello_world])
        .mount(
            "/api/v1/ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "/api/v1/openapi.json".to_string(),
                ..Default::default()
            }),
        )
}
