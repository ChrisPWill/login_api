#[macro_use]
extern crate rouille;

pub mod routes;

use routes::auth_routes;

fn main() {
    rouille::start_server("localhost:8000", move |request| {
        auth_routes(&request)
    });
}
