use super::{DalConnection, schema::{auth_log, auth_tokens, users}};
use chrono::{DateTime, Utc};
use diesel;
use diesel::{prelude::*,
             result::{DatabaseErrorKind, Error::DatabaseError,
                      Error::NotFound}};
use uuid::Uuid;

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Identifiable, Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub date_created: DateTime<Utc>,
}

pub enum CreateUserError {
    EmailExists,
    OtherDbError(diesel::result::Error),
}

pub fn create_user<'a>(
    connection: &DalConnection,
    new_user: &'a NewUser,
) -> Result<User, CreateUserError> {
    let pg_connection = &connection.pg_connection;
    let result = diesel::insert_into(users::table)
        .values(new_user)
        .get_result(pg_connection);
    match result {
        Ok(user) => Ok(user),
        Err(DatabaseError(error, message)) => match error {
            DatabaseErrorKind::UniqueViolation => {
                Err(CreateUserError::EmailExists)
            }
            _ => Err(CreateUserError::OtherDbError(DatabaseError(
                error,
                message,
            ))),
        },
        Err(error) => Err(CreateUserError::OtherDbError(error)),
    }
}

pub enum GetUserError {
    UserNotFound,
    OtherDbError(diesel::result::Error),
}

pub fn get_user_by_email(
    connection: &DalConnection,
    email_to_check: &str,
) -> Result<User, GetUserError> {
    use super::schema::users::dsl::*;

    let pg_connection = &connection.pg_connection;
    let result = users
        .filter(email.eq(email_to_check))
        .first(pg_connection);

    match result {
        Ok(user) => Ok(user),
        Err(NotFound) => Err(GetUserError::UserNotFound),
        Err(error) => Err(GetUserError::OtherDbError(error)),
    }
}

#[derive(Identifiable, Insertable, Queryable)]
#[table_name = "auth_tokens"]
pub struct AuthToken {
    pub id: i64,
    pub user_id: i32,
    pub token: Uuid,
    pub date_created: DateTime<Utc>,
    pub date_expired: DateTime<Utc>,
    pub token_type: String,
}

pub enum CreateAuthTokenError {
    OtherDbError(diesel::result::Error),
}

pub fn create_token<'a>(
    connection: &DalConnection,
    new_token: &'a AuthToken,
) -> Result<AuthToken, CreateAuthTokenError> {
    let pg_connection = &connection.pg_connection;
    let result = diesel::insert_into(auth_tokens::table)
        .values(new_token)
        .get_result(pg_connection);
    match result {
        Ok(token) => Ok(token),
        Err(error) => Err(CreateAuthTokenError::OtherDbError(error)),
    }
}

#[derive(Identifiable, Insertable, Queryable)]
#[table_name = "auth_log"]
pub struct AuthLog {
    pub id: i64,
    pub email: String,
    pub success: bool,
    pub ip_address: String,
    pub user_agent: String,
    pub date_created: DateTime<Utc>,
}

pub enum CreateAuthLogError {
    OtherDbError(diesel::result::Error),
}

pub fn create_auth_log<'a>(
    connection: &DalConnection,
    new_log: &'a AuthLog,
) -> Result<AuthLog, CreateAuthLogError> {
    let pg_connection = &connection.pg_connection;
    let result = diesel::insert_into(auth_log::table)
        .values(new_log)
        .get_result(pg_connection);
    match result {
        Ok(token) => Ok(token),
        Err(error) => Err(CreateAuthLogError::OtherDbError(error)),
    }
}
