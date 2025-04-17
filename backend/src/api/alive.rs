use rocket::http::Status;
use rocket_okapi::openapi;


#[openapi(tag = "Alive")]
#[get("/alive")]
pub fn is_alive() -> Status {
    Status::Ok
}