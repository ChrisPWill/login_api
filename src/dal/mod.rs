pub mod schema;

use diesel::{
    Connection,
    pg::PgConnection,
};

pub fn establish_connection(database_url: &str) -> PgConnection {
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
