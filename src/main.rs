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

use dotenv::dotenv;
use rouille::{Request, Response};
use std::env;

fn main() {
    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    env::var("HMAC_HASH").expect("HMAC_HASH must be set");

    rouille::start_server("localhost:8000", move |request| {
        routes(&request)
    });
}

fn routes(request: &Request) -> Response {
    router!(
        request,
        (GET) (/) => {
            Response::empty_404()
        },
        _ => {
            if let Some(v1_request) = request.remove_prefix("/v1") {
                v1::v1_routes(&v1_request)
            } else {
                Response::empty_404()
            }
        },
    )
}
