use rouille::{Response, Request};

pub fn auth_routes(request: &Request) -> Response {
    router!(
        request,
        (GET) (/) => {
            Response::empty_404()
        },
        _ => Response::empty_404(),
    )
}
