use rocket::{get, post};
use rocket_contrib::{json::Json, uuid::Uuid};

use crate::{
    models::{NewUser, User},
    repositories::users,
    DbConn,
};

use super::{FromResult, JsonResponse};

#[get("/user")]
pub fn index<'a>(connection: DbConn) -> JsonResponse<'a, Vec<User>> {
    JsonResponse::from_result(users::all(&connection))
}

#[get("/user/<_id>")]
pub fn get<'a>(connection: DbConn, _id: Uuid) -> JsonResponse<'a, User> {
    JsonResponse::from_result(users::get(_id.into_inner(), &connection))
}

#[post("/user", format = "application/json", data = "<user>")]
pub fn store<'a>(user: Json<NewUser>, conn: DbConn) -> JsonResponse<'a, User> {
    let new_user = user.into_inner().hash_password();
    JsonResponse::from_result(users::insert(new_user, &conn))
}
