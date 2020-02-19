use rocket::get;
use rocket_contrib::json::Json;
use rocket_contrib::uuid::Uuid;

use crate::models::User;
use crate::repositories::users;
use crate::DbConn;

use super::{error_status, IntoJsonResponse, JsonResponse};

#[get("/user")]
pub fn all_users(connection: DbConn) -> JsonResponse<Vec<User>> {
    //    fromResultSet::<Vec<User>>(users::all(&connection))

    JsonResponse::from_query_result(users::all(&connection))
}

#[get("/user/<_id>")]
pub fn user(connection: DbConn, _id: Uuid) -> JsonResponse<User> {
    users::get(_id.into_inner(), &connection)
        .map(|user| Json(user))
        .map_err(|error| error_status(error))
}
