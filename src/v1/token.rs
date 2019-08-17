use dal::DalConnection;
use handlers;
use rouille::{input::json::JsonError, input::json_input, Request, Response};
use v1::models::{
    response::SingleErrorResponse,
    token::{CreateTokenRequest, CreateTokenResponse},
};
use validator::Validate;

pub fn token_routes(request: &Request, connection: &DalConnection) -> Response {
    router!(
        request,
        (POST) [""] => create_token(&request, &connection),
        _ => Response::empty_404(),
    )
}

fn create_token(request: &Request, connection: &DalConnection) -> Response {
    let body: CreateTokenRequest = match json_input(request) {
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
    // Validate fields
    match body.validate() {
        Ok(_) => (),
        Err(e) => {
            let mut response = Response::json(&e);
            response.status_code = 422;
            return response;
        }
    }

    match handlers::user::create_token(
        connection,
        &body.email,
        &body.password,
        &request.remote_addr().ip().to_string(),
        request.header("User-Agent").unwrap(),
    ) {
        Ok(token) => {
            let mut response =
                Response::json(&CreateTokenResponse { token: token });
            response.status_code = 201;
            response
        }
        Err(handlers::user::CreateTokenError::OtherDbError(err)) => {
            panic!("Unexpected database error: {}", err);
        }
        Err(_) => {
            let mut response = Response::json(&SingleErrorResponse {
                error: "Unauthorized".to_owned(),
            });
            response.status_code = 401;
            return response;
        }
    }
}
