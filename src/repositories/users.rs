use diesel::{self, prelude::*};
use uuid::Uuid;

use crate::{
    database::{ConnectionType, DbConn},
    models::{NewUser, User},
    schema::users,
};

pub type Connection = <DbConn as ConnectionType>::Connection;

pub fn all(connection: &Connection) -> QueryResult<Vec<User>> {
    users::table.load::<User>(&*connection)
}

pub fn get(id: Uuid, connection: &Connection) -> QueryResult<User> {
    users::table.find(id).get_result::<User>(connection)
}

pub fn insert(user: NewUser, connection: &Connection) -> QueryResult<User> {
    diesel::insert_into(users::table)
        .values(&user)
        .get_result(connection)
}

pub fn update(id: Uuid, user: User, connection: &Connection) -> QueryResult<User> {
    diesel::update(users::table.find(id))
        .set(&user)
        .get_result(connection)
}

pub fn delete(id: Uuid, connection: &Connection) -> QueryResult<usize> {
    diesel::delete(users::table.find(id)).execute(connection)
}
