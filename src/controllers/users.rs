use diesel::result::Error;
use rocket::get;
use rocket::handler::Outcome;
use rocket::http::Status;
use rocket_contrib::json::Json;
use rocket_contrib::uuid::Uuid;

use crate::models::User;
use crate::repositories::users;
use crate::DbConn;

type Response<U> = Result<Json<U>, Outcome<'static>>;

fn error_status(error: Error) -> Outcome<'static> {
    Outcome::failure(match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    })
}

#[get("/user")]
pub fn all_users(connection: DbConn) -> Response<Vec<User>> {
    users::all(&connection)
        .map(|_users| Json(_users))
        .map_err(|error| error_status(error))
}

#[get("/user/<_id>")]
pub fn user(connection: DbConn, _id: Uuid) -> Response<User> {
    users::get(_id.into_inner(), &connection)
        .map(|user| Json(user))
        .map_err(|error| error_status(error))
}
