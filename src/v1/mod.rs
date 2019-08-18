pub mod models;
pub mod token;
pub mod user;

use dal::DalConnection;
use rouille::{Request, Response};

pub fn routes(request: &Request, connection: &DalConnection) -> Response {
    router!(
        request,
        (GET) (/) => {
            Response::empty_404()
        },
        _ => {
            if let Some(user_request) = request.remove_prefix("/user") {
                user::routes(&user_request, connection)
            } else if let Some(token_request) = request.remove_prefix("/token") {
                token::routes(&token_request, connection)
            } else {
                Response::empty_404()
            }
        },
    )
}
