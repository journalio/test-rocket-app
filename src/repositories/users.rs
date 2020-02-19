use diesel;
use diesel::prelude::*;
use uuid::Uuid;

use crate::models::{NewUser, User};
use crate::schema::users;

pub fn all(connection: &PgConnection) -> QueryResult<Vec<User>> {
    users::table.load::<User>(&*connection)
}

pub fn get(id: Uuid, connection: &PgConnection) -> QueryResult<User> {
    users::table.find(id).get_result::<User>(connection)
}

pub fn insert(user: NewUser, connection: &PgConnection) -> QueryResult<User> {
    diesel::insert_into(users::table)
        .values(&user)
        .get_result(connection)
}

pub fn update(id: Uuid, user: User, connection: &PgConnection) -> QueryResult<User> {
    diesel::update(users::table.find(id))
        .set(&user)
        .get_result(connection)
}

pub fn delete(id: Uuid, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(users::table.find(id)).execute(connection)
}
