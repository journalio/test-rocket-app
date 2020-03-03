use diesel::result::Error;
use rocket::{handler::Outcome, http::Status};
use rocket_contrib::json::Json;

///controller definitions
pub mod users;

/// Internal types used by controllers
type JsonResponse<'r, U> = Result<Json<U>, Outcome<'r>>;

trait FromResult<T, E> {
    fn from_result(r: Result<T, E>) -> Self;
}

impl<'r, T> FromResult<T, Error> for JsonResponse<'r, T> {
    fn from_result(r: Result<T, Error>) -> Self {
        r.map(Json).map_err(error_status)
    }
}

/// Generates an error response
fn error_status(error: Error) -> Outcome<'static> {
    Outcome::failure(match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    })
}
