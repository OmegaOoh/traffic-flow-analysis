use rocket::http::Status;
use rocket_okapi::openapi;


#[openapi(tag = "Alive")]
#[get("/alive")]
pub fn is_alive() -> Status {
    Status::Ok
}


#[cfg(test)]
mod test {
    use rocket::http::Status;
    use crate::api::alive::is_alive;

    #[test]
    fn test_is_alive() {
        assert_eq!(is_alive(), Status::Ok)
    }
}