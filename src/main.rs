#[macro_use]
extern crate rouille;
extern crate serde;
#[macro_use]
extern crate serde_derive;

pub mod v1;

use rouille::{Request, Response};

fn main() {
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
