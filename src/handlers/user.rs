extern crate easy_password;

use self::easy_password::bcrypt::{hash_password, verify_password};
use base64;
use chrono::{prelude::*, Duration};
use dal;
use dal::{
    auth::{
        AuthLog, CreateAuthLogError, CreateAuthTokenError, GetAuthTokenError,
        NewAuthLog, NewAuthToken,
    },
    users::{CreateUserError, GetUserError, NewUser, User},
    DalConnection,
};
use diesel;
use jwt;
use rand::Rng;
use std::env;

#[derive(Deserialize, Serialize)]
pub struct AuthTokenClaims {
    pub token_id: i64,
    pub user_id: i64,
    pub email: String,
    pub token: String,
    pub exp: usize,
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
    dal::auth::create_auth_log(connection, &auth_log)
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

    let date_created = Utc::now();
    let date_expired = Utc::now() + Duration::hours(1);
    let new_token = NewAuthToken {
        user_id: user.id,
        token: rand::thread_rng().gen::<[u8; 16]>().to_vec(),
        date_created: date_created,
        date_expired: date_expired,
        token_type: "authentication",
    };

    if password_valid {
        match dal::auth::create_token(connection, &new_token) {
            Ok(token) => Ok(jwt::encode(
                &jwt::Header::default(),
                &AuthTokenClaims {
                    token_id: token.id,
                    user_id: token.user_id,
                    email: email.to_string(),
                    token: base64::encode(&new_token.token),
                    exp: date_expired.timestamp() as usize,
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

pub fn decode_jwt_token(
    token_string: &str,
) -> Result<jwt::TokenData<AuthTokenClaims>, jwt::errors::Error> {
    jwt::decode::<AuthTokenClaims>(
        token_string,
        env::var("JWT_SECRET")
            .expect("JWT_SECRET must be set")
            .as_bytes(),
        &jwt::Validation {
            leeway: 60,
            ..Default::default()
        },
    )
}

#[derive(Debug)]
pub enum VerifyTokenError {
    TokenMismatch,
    UserMismatch,
    JwtError(jwt::errors::Error),
    GetAuthTokenError(GetAuthTokenError),
}

pub fn verify_token(
    connection: &DalConnection,
    token_string: &str,
) -> Result<(), VerifyTokenError> {
    let jwt_token = match decode_jwt_token(token_string) {
        Ok(jwt_token) => jwt_token,
        Err(error) => {
            return Err(VerifyTokenError::JwtError(error));
        }
    };

    let auth_token_from_db = match dal::auth::get_auth_token(
        connection,
        jwt_token.claims.token_id,
    ) {
        Ok(auth_token_from_db) => auth_token_from_db,
        Err(error) => {
            return Err(VerifyTokenError::GetAuthTokenError(error));
        }
    };

    let encoded_token = base64::encode(&auth_token_from_db.token);
    let user_ids_match = jwt_token.claims.user_id == auth_token_from_db.user_id;
    let tokens_match = jwt_token.claims.token == encoded_token;
    match (user_ids_match, tokens_match) {
        (true, true) => Ok(()),
        (false, _) => Err(VerifyTokenError::UserMismatch),
        (_, false) => Err(VerifyTokenError::TokenMismatch),
    }
}
