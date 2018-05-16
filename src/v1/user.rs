use rouille::{Request, Response};

pub fn user_routes(request: &Request) -> Response {
    router!(
        request,
        (GET) (/) => {
            Response::empty_404()
        },
        _ => Response::empty_404(),
    )
}
