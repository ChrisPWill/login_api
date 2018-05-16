pub mod models;
pub mod user;

use dal::DalConnection;
use rouille::{Request, Response};

pub fn v1_routes(request: &Request, connection: &DalConnection) -> Response {
    router!(
        request,
        (GET) (/) => {
            Response::empty_404()
        },
        _ => {
            if let Some(user_request) = request.remove_prefix("/user") {
                user::user_routes(&user_request, &connection)
            } else {
                Response::empty_404()
            }
        },
    )
}
