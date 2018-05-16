use chrono::{DateTime, Utc};
use diesel;
use diesel::prelude::*;
use super::{
    DalConnection,
    schema::users,
};

#[derive(Insertable)]
#[table_name="users"]
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

pub fn create_user<'a>(connection: &DalConnection, new_user: &'a NewUser) -> User {
    let pg_connection = &connection.pg_connection;
    diesel::insert_into(users::table)
        .values(new_user)
        .get_result(pg_connection)
        .expect("Error saving new user")
}
