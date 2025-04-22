use rocket_db_pools::{sqlx, Database};
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::OpenApiError;

#[derive(Database)]
#[database("mysql_logs")]
pub struct Logs(pub sqlx::MySqlPool);

impl<'r> OpenApiFromRequest<'r> for &'r Logs {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> Result<RequestHeaderInput, OpenApiError> {
        // Database connections are not parameters
        Ok(RequestHeaderInput::None)
    }
}