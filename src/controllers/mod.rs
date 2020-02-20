use diesel::result::Error;
use rocket::{handler::Outcome, http::Status};
use rocket_contrib::json::Json;

///controller definitions
pub mod users;

/// Internal types used by controllers
type JsonResponse<U> = Result<Json<U>, Outcome<'static>>;

trait IntoJsonResponse<U> {
    fn from_query_result(result: Result<U, Error>) -> JsonResponse<U>;
}

impl<U> IntoJsonResponse<U> for JsonResponse<U> {
    fn from_query_result(result: Result<U, Error>) -> JsonResponse<U> {
        result.map(Json).map_err(error_status)
    }
}

/// Generates an error response
fn error_status(error: Error) -> Outcome<'static> {
    Outcome::failure(match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    })
}
