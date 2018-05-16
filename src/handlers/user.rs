extern crate easy_password;

use self::easy_password::bcrypt::hash_password;
use dal;
use dal::{DalConnection, users::{NewUser, User}};
use std::env;

pub fn create_user(
    connection: &DalConnection,
    email: &str,
    password: &str,
) -> User {
    let hashed_password = hash_password(
        password,
        env::var("HMAC_HASH")
            .expect("HMAC_HASH must be set")
            .as_bytes(),
        12,
    ).unwrap();

    let new_user = NewUser {
        email: email,
        password: &hashed_password,
    };
    dal::users::create_user(connection, &new_user)
}
