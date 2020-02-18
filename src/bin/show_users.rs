extern crate diesel;

use self::diesel::prelude::*;
use test_rocket_app::establish_connection;
use test_rocket_app::models::User;

fn main() {
    use test_rocket_app::schema::users::dsl::*;

    let connection = establish_connection();
    let results = users
        .limit(5)
        .load::<User>(&connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{}", user.full_name);
        println!("----------\n");
        println!("{}", user.email);
    }
}
