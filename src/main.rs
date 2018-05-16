extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate rouille;
extern crate serde;
#[macro_use]
extern crate serde_derive;

pub mod dal;
pub mod handlers;
pub mod v1;

use dal::DalConnection;
use diesel::{Connection, pg::PgConnection};
use dotenv::dotenv;
use rouille::{Request, Response};
use std::env;

fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let _hmac_hash = env::var("HMAC_HASH").expect("HMAC_HASH must be set");

    rouille::start_server("localhost:8000", move |request| {
        let connection = DalConnection::new(
            PgConnection::establish(&db_url).expect("Error connecting to DB!"),
        );

        routes(&request, &connection)
    });
}

fn routes(request: &Request, connection: &DalConnection) -> Response {
    router!(
        request,
        (GET) (/) => {
            Response::empty_404()
        },
        _ => {
            if let Some(v1_request) = request.remove_prefix("/v1") {
                v1::v1_routes(&v1_request, &connection)
            } else {
                Response::empty_404()
            }
        },
    )
}
