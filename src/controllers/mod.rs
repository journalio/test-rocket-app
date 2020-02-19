use diesel::result::Error;
use rocket::handler::Outcome;
use rocket::http::Status;
use rocket_contrib::json::Json;

pub mod users;

type JsonResponse<U> = Result<Json<U>, Outcome<'static>>;

fn error_status(error: Error) -> Outcome<'static> {
    Outcome::failure(match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    })
}

trait IntoJsonResponse<U> {
    fn from_query_result(result: Result<U, Error>) -> JsonResponse<U>;
}

impl<U> IntoJsonResponse<U> for JsonResponse<U> {
    fn from_query_result(result: Result<U, Error>) -> JsonResponse<U> {
        result.map(Json).map_err(error_status)
    }
}
