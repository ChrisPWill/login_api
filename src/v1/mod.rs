pub mod models;
pub mod user;

use rouille::{Request, Response};

pub fn v1_routes(request: &Request) -> Response {
    router!(
        request,
        (GET) (/) => {
            Response::empty_404()
        },
        _ => {
            if let Some(user_request) = request.remove_prefix("/user") {
                user::user_routes(&user_request)
            } else {
                Response::empty_404()
            }
        },
    )
}
