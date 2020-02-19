use rocket::{post, get};
use rocket_contrib::uuid::Uuid;
use rocket_contrib::json::Json;

use crate::models::{NewUser, User};
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

#[post("/user", format = "application/json", data = "<user>")]
pub fn post(user: Json<NewUser>, conn: DbConn) -> JsonResponse<User> {
    let user = user.into_inner();

    let new_user = user.hash_password();

    JsonResponse::from_query_result(users::insert(new_user, &conn))
}
