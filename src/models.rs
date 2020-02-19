use diesel::sql_types::Timestamp;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::schema::users;

#[derive(Queryable, Serialize, AsChangeset, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub full_name: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub full_name: String,
    pub email: String,
    pub password_hash: String,
}

impl NewUser {
    pub fn hash_password(self) -> Self {
        use crypto::bcrypt::bcrypt;
        use base64::encode;
        
        #[inline]
        fn hash_password(s: String) -> String {
            let mut hash = vec![0; 24];
            let salt = b"SALTYSALTYSALTY!";

            assert!(salt.len() == 16);

            bcrypt(1, salt, &s.as_bytes(), &mut hash);
            encode(&hash)
        }

        // This is a destructure of `self`.
        let Self { full_name, email, password_hash } = self;
        
        let password_hash = hash_password(password_hash);

        Self {
            full_name,
            email,
            password_hash,
        }
    }
}

#[derive(Queryable)]
pub struct Tag {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub color: u8,
    pub created_at: Timestamp,
    pub created_by: Uuid,
    pub updated_at: Timestamp,
}

#[derive(Queryable)]
pub struct Item {
    pub id: Uuid,
    pub title: String,
    pub item_type: String,
    pub created_at: Timestamp,
    pub created_by: Uuid,
    pub updated_at: Timestamp,
    pub owned_by: Uuid,
}

#[derive(Queryable)]
pub struct Todo {
    pub id: Uuid,
    pub content: String,
    pub done: bool,
}
