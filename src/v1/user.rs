use rouille::{Request, Response, input::json::JsonError, input::json_input};
use v1::models::{response::SingleErrorResponse,
                 user::{CreateUserRequest, CreateUserResponse}};

pub fn user_routes(request: &Request) -> Response {
    router!(
        request,
        (POST) [""] => create_user(&request),
        _ => Response::empty_404(),
    )
}

fn create_user(request: &Request) -> Response {
    let body: CreateUserRequest = match json_input(request) {
        Ok(body) => body,
        Err(JsonError::WrongContentType)
        | Err(JsonError::IoError(_))
        | Err(JsonError::ParseError(_)) => {
            let mut response = Response::json(&SingleErrorResponse {
                error: "Body format error".to_owned(),
            });
            response.status_code = 400;
            return response;
        }
        _ => panic!("Body should only be extracted once."),
    };
    Response::json(&CreateUserResponse {
        email: body.email,
    })
}