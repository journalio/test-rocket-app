use rocket::{get, post};
use rocket_contrib::{json::Json, uuid::Uuid};

use crate::{
    models::{NewUser, User},
    repositories::users,
    DbConn,
};

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
pub fn store(user: Json<NewUser>, conn: DbConn) -> JsonResponse<User> {
    let new_user = user.into_inner().hash_password();
    JsonResponse::from_query_result(users::insert(new_user, &conn))
}
