use super::{schema::users, DalConnection};
use chrono::{DateTime, Utc};
use diesel::{
    self,
    prelude::*,
    result::{
        DatabaseErrorKind,
        Error::{DatabaseError, NotFound},
    },
};

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Identifiable, Queryable)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub password: String,
    pub date_created: DateTime<Utc>,
    pub date_modified: DateTime<Utc>,
}

pub enum CreateUserError {
    EmailExists,
    OtherDbError(diesel::result::Error),
}

pub fn create_user<'a>(
    connection: &DalConnection,
    new_user: &NewUser<'a>,
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
                error, message,
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
    let result = users.filter(email.eq(email_to_check)).first(pg_connection);

    match result {
        Ok(user) => Ok(user),
        Err(NotFound) => Err(GetUserError::UserNotFound),
        Err(error) => Err(GetUserError::OtherDbError(error)),
    }
}

pub fn get_user_by_id(
    connection: &DalConnection,
    user_id: i64,
) -> Result<User, GetUserError> {
    use super::schema::users::dsl::*;

    let pg_connection = &connection.pg_connection;
    let result = users.filter(id.eq(user_id)).first(pg_connection);

    match result {
        Ok(user) => Ok(user),
        Err(NotFound) => Err(GetUserError::UserNotFound),
        Err(error) => Err(GetUserError::OtherDbError(error)),
    }
}
