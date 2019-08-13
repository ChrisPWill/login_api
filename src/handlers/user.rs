extern crate easy_password;

use self::easy_password::bcrypt::{hash_password, verify_password};
use chrono::{prelude::*, Duration};
use dal;
use dal::{
    users::{
        AuthLog, CreateAuthLogError, CreateAuthTokenError, CreateUserError,
        GetUserError, NewAuthLog, NewAuthToken, NewUser, User,
    },
    DalConnection,
};
use diesel;
use jwt::{encode, Header};
use std::env;
use uuid::Uuid;

#[derive(Serialize)]
pub struct AuthTokenClaims {
    pub user_id: i64,
    pub email: String,
    pub token: String,
}

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
    )
    .expect("Parameters should be valid");

    let new_user = NewUser {
        email: email,
        password: &hashed_password,
    };
    dal::users::create_user(connection, &new_user)
}

fn log_auth_attempt(
    connection: &DalConnection,
    email: &str,
    ip_address: &str,
    user_agent: &str,
    success: bool,
) -> Result<AuthLog, CreateAuthLogError> {
    let auth_log = NewAuthLog {
        email: email,
        success: success,
        ip_address: ip_address,
        user_agent: user_agent,
        date_created: Utc::now(),
    };
    dal::users::create_auth_log(connection, &auth_log)
}

pub enum LoginError {
    UserNotFound,
    WrongPassword,
    OtherDbError(diesel::result::Error),
}

pub fn login(
    connection: &DalConnection,
    email: &str,
    password: &str,
    ip_address: &str,
    user_agent: &str,
) -> Result<String, LoginError> {
    let user = match dal::users::get_user_by_email(connection, email) {
        Ok(user) => user,
        Err(error) => {
            match log_auth_attempt(
                connection, email, ip_address, user_agent, false,
            ) {
                Ok(_) => (),
                Err(log_error) => match log_error {
                    CreateAuthLogError::OtherDbError(db_error) => {
                        return Err(LoginError::OtherDbError(db_error));
                    }
                },
            }
            match error {
                GetUserError::UserNotFound => {
                    return Err(LoginError::UserNotFound);
                }
                GetUserError::OtherDbError(db_error) => {
                    return Err(LoginError::OtherDbError(db_error));
                }
            }
        }
    };

    let password_valid = verify_password(
        password,
        &user.password,
        env::var("HMAC_HASH")
            .expect("HMAC_HASH must be set")
            .as_bytes(),
    )
    .expect("Parameters should be valid");
    match log_auth_attempt(
        connection,
        email,
        ip_address,
        user_agent,
        password_valid,
    ) {
        Ok(_) => (),
        Err(log_error) => match log_error {
            CreateAuthLogError::OtherDbError(db_error) => {
                return Err(LoginError::OtherDbError(db_error));
            }
        },
    }

    let new_token = NewAuthToken {
        user_id: user.id,
        token: Uuid::new_v4(),
        date_created: Utc::now(),
        date_expired: (Utc::now() + Duration::hours(1)),
        token_type: "authentication",
    };

    if password_valid {
        match dal::users::create_token(connection, &new_token) {
            Ok(_) => Ok(encode(
                &Header::default(),
                &AuthTokenClaims {
                    user_id: new_token.user_id,
                    email: email.to_string(),
                    token: new_token.token.to_string(),
                },
                env::var("JWT_SECRET")
                    .expect("JWT_SECRET must be set")
                    .as_bytes(),
            )
            .unwrap()),
            Err(error) => match error {
                CreateAuthTokenError::OtherDbError(db_error) => {
                    Err(LoginError::OtherDbError(db_error))
                }
            },
        }
    } else {
        Err(LoginError::WrongPassword)
    }
}
