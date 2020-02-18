#![feature(proc_macro_hygiene, decl_macro)]

extern crate diesel;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use diesel::prelude::*;
use rocket_contrib::json::Json;
use rocket_contrib::uuid::Uuid;

use test_rocket_app::establish_connection;
use test_rocket_app::models::*;

#[get("/user/<_id>")]
fn user(_id: Uuid) -> Json<User> {
    use test_rocket_app::schema::users::dsl::*;

    let connection = establish_connection();
    let user = users
        .filter(id.eq(_id.into_inner()))
        .first(&connection)
        .expect("User not found");
    Json(user)
}

#[catch(404)]
fn not_found() -> String {
    "{\"status\": \"Not Found\", \"Message\": \"Page not found\"}".to_string()
}

fn main() {
    rocket::ignite()
        .mount("/api", routes![user])
        .register(catchers![not_found])
        .launch();
}
