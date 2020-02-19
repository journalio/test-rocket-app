use diesel;
use diesel::prelude::*;
use uuid::Uuid;

use crate::models::User;
use crate::schema::users;

pub fn all(connection: &PgConnection) -> QueryResult<Vec<User>> {
    users::table.load::<User>(&*connection)
}

pub fn get(id: Uuid, connection: &PgConnection) -> QueryResult<User> {
    users::table.find(id).get_result::<User>(connection)
}

//pub fn insert(person: User, connection: &PgConnection) -> QueryResult<User> {
//    diesel::insert_into(users::table)
//        .values(&NewUser::from_person(person))
//        .get_result(connection)
//}

pub fn update(id: Uuid, person: User, connection: &PgConnection) -> QueryResult<User> {
    diesel::update(users::table.find(id))
        .set(&person)
        .get_result(connection)
}

pub fn delete(id: Uuid, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(users::table.find(id)).execute(connection)
}