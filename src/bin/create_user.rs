extern crate diesel;

use test_rocket_app::establish_connection;
use test_rocket_app::models::{NewUser, User};

use self::diesel::prelude::*;

fn main() {
    use test_rocket_app::schema::users::dsl::*;

    let connection = establish_connection();
    let new_user = NewUser {
        full_name: "Wesley Klop",
        email: "wesley19097@gmail.com",
        password_hash: "blabblablabcrypt nog nodig",
    };
    let created_user: User = diesel::insert_into(users)
        .values(new_user)
        .get_result(&connection)
        .expect("Failed to insert user");

    println!("{}", created_user.full_name);
    println!("----------\n");
    println!("{}", created_user.email);
}
