pub mod auth;

use rouille::{Response, Request};

pub fn v1_routes(request: &Request) -> Response {
    router!(
        request,
        (GET) (/) => {
            Response::empty_404()
        },
        _ => {
            if let Some(auth_request) = request.remove_prefix("/auth") {
                auth::auth_routes(&auth_request)
            } else {
                Response::empty_404()
            }
        },
    )
}
