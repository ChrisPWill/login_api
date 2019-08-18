pub mod auth;
pub mod schema;
pub mod users;

use diesel::{pg::PgConnection, Connection};

pub fn establish_connection(database_url: &str) -> PgConnection {
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

/// Thin wrapper to reduce refactoring work should connection code get changed
pub struct DalConnection {
    pub pg_connection: PgConnection,
}

impl DalConnection {
    pub fn new(connection: PgConnection) -> DalConnection {
        DalConnection {
            pg_connection: connection,
        }
    }
}
