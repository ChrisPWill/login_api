extern crate easy_password;

use self::easy_password::bcrypt::hash_password;
use dal;
use dal::{DalConnection, users::{CreateUserError, NewUser, User}};
use std::env;

pub fn create_user(
    connection: &DalConnection,
    email: &str,
    password: &str,
) -> Result<User, CreateUserError> {
    if dal::users::get_user_by_email(connection, email).is_ok() {
        return Err(CreateUserError::EmailExists);
    }

    let hashed_password = hash_password(
        password,
        env::var("HMAC_HASH")
            .expect("HMAC_HASH must be set")
            .as_bytes(),
        12,
    ).expect("Parameters should be valid");

    let new_user = NewUser {
        email: email,
        password: &hashed_password,
    };
    dal::users::create_user(connection, &new_user)
}
