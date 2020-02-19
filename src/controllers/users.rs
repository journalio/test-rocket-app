use rocket::get;
use rocket_contrib::uuid::Uuid;

use crate::models::User;
use crate::repositories::users;
use crate::DbConn;

use super::{IntoJsonResponse, JsonResponse};

#[get("/user")]
pub fn index(connection: DbConn) -> JsonResponse<Vec<User>> {
    JsonResponse::from_query_result(users::all(&connection))
}

#[get("/user/<_id>")]
pub fn get(connection: DbConn, _id: Uuid) -> JsonResponse<User> {
    JsonResponse::from_query_result(users::get(_id.into_inner(), &connection))
}
