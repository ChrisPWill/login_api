use dal::{DalConnection, users::CreateUserError};
use handlers;
use rouille::{Request, Response, input::json::JsonError, input::json_input};
use v1::models::{response::SingleErrorResponse,
                 user::{CreateUserRequest, CreateUserResponse, LoginRequest}};
use validator::Validate;

pub fn user_routes(request: &Request, connection: &DalConnection) -> Response {
    router!(
        request,
        (POST) [""] => create_user(&request, &connection),
        (POST) ["/login"] => login(&request, &connection),
        _ => Response::empty_404(),
    )
}

fn create_user(request: &Request, connection: &DalConnection) -> Response {
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
    // Validate other fields
    match body.validate() {
        Ok(_) => (),
        Err(e) => {
            let mut response = Response::json(&e);
            response.status_code = 422;
            return response;
        }
    }

    let user_result =
        handlers::user::create_user(connection, &body.email, &body.password);
    match user_result {
        Ok(user) => {
            let mut response = Response::json(&CreateUserResponse {
                id: user.id,
                email: user.email,
                date_created: user.date_created,
            });
            response.status_code = 201;
            response
        }
        Err(CreateUserError::EmailExists) => {
            let mut response = Response::json(&SingleErrorResponse {
                error: "Email already registered".to_owned(),
            });
            response.status_code = 409;
            response
        }
        Err(CreateUserError::OtherDbError(err)) => {
            panic!("Unexpected database error: {}", err);
        }
    }
}

fn login(request: &Request, connection: &DalConnection) -> Response {
    let body: LoginRequest = match json_input(request) {
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

    match handlers::user::login(
        connection,
        &body.email,
        &body.password,
        &request.remote_addr().ip().to_string(),
        request.header("User-Agent").unwrap(),
    ) {
        Ok(token) => {
            let mut response = Response::text(token);
            response.status_code = 201;
            response
        }
        Err(handlers::user::LoginError::OtherDbError(err)) => {
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
