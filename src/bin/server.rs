#![feature(proc_macro_hygiene, decl_macro)]

extern crate diesel;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use dotenv::dotenv;
use test_rocket_app::controllers;
use test_rocket_app::init_pool;

#[catch(404)]
fn not_found() -> String {
    "{\"status\": \"Not Found\", \"Message\": \"Page not found\"}".to_string()
}

fn main() {
    dotenv().ok();
    rocket::ignite()
        .manage(init_pool())
        .mount(
            "/api",
            routes![controllers::users::all_users, controllers::users::user],
        )
        .register(catchers![not_found])
        .launch();
}
