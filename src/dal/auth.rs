use super::{
    schema::{auth_log, auth_tokens},
    DalConnection,
};
use chrono::{DateTime, Utc};
use diesel::{self, prelude::*, result::Error::NotFound};

#[derive(Insertable)]
#[table_name = "auth_tokens"]
pub struct NewAuthToken<'a> {
    pub user_id: i64,
    pub token: Vec<u8>,
    pub date_created: DateTime<Utc>,
    pub date_expired: DateTime<Utc>,
    pub token_type: &'a str,
}

#[derive(Identifiable, Queryable)]
#[table_name = "auth_tokens"]
pub struct AuthToken {
    pub id: i64,
    pub user_id: i64,
    pub token: Vec<u8>,
    pub date_created: DateTime<Utc>,
    pub date_expired: DateTime<Utc>,
    pub token_type: String,
}

pub enum CreateAuthTokenError {
    OtherDbError(diesel::result::Error),
}

pub fn create_token<'a>(
    connection: &DalConnection,
    new_token: &NewAuthToken<'a>,
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

#[derive(Debug)]
pub enum GetAuthTokenError {
    AuthTokenNotFound,
    OtherDbError(diesel::result::Error),
}

pub fn get_auth_token(
    connection: &DalConnection,
    token_id: i64,
) -> Result<AuthToken, GetAuthTokenError> {
    use super::schema::auth_tokens::dsl::*;

    let pg_connection = &connection.pg_connection;
    match auth_tokens.filter(id.eq(token_id)).first(pg_connection) {
        Ok(user) => Ok(user),
        Err(NotFound) => Err(GetAuthTokenError::AuthTokenNotFound),
        Err(error) => Err(GetAuthTokenError::OtherDbError(error)),
    }
}

#[derive(Insertable)]
#[table_name = "auth_log"]
pub struct NewAuthLog<'a> {
    pub email: &'a str,
    pub success: bool,
    pub ip_address: &'a str,
    pub user_agent: &'a str,
    pub date_created: DateTime<Utc>,
}

#[derive(Identifiable, Queryable)]
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
    new_log: &NewAuthLog<'a>,
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
