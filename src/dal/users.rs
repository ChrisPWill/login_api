use super::{DalConnection, schema::users};
use chrono::{DateTime, Utc};
use diesel;
use diesel::{prelude::*, result::{DatabaseErrorKind, Error::DatabaseError}};

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
