use rouille::{Response, Request};

pub fn v1_routes(request: &Request) -> Response {
    router!(
        request,
        (GET) (/) => {
            Response::empty_404()
        },
        _ => Response::empty_404(),
    )
}
