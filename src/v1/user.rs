use dal::{users::CreateUserError, DalConnection};
use handlers;
use handlers::user::VerifyTokenError;
use jwt;
use rouille::{input::json::JsonError, input::json_input, Request, Response};
use v1::models::{
    response::SingleErrorResponse,
    user::{
        CreateUserRequest, CreateUserResponse, LoginRequest, LoginResponse,
        ValidateTokenRequest, ValidateTokenResponse,
    },
};
use validator::Validate;

pub fn user_routes(request: &Request, connection: &DalConnection) -> Response {
    router!(
        request,
        (POST) [""] => create_user(&request, &connection),
        (POST) ["/login"] => login(&request, &connection),
        (POST) ["/validate"] => validate_token(&request, &connection),
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
            let mut response = Response::json(&LoginResponse { token: token });
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

fn validate_token(request: &Request, connection: &DalConnection) -> Response {
    let body: ValidateTokenRequest = match json_input(request) {
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

    match handlers::user::verify_token(connection, &body.token) {
        Ok((user_id, email)) => {
            let mut response =
                Response::json(&ValidateTokenResponse { user_id, email });
            response.status_code = 200;
            response
        }
        Err(VerifyTokenError::TokenMismatch) => {
            let mut response = Response::json(&SingleErrorResponse {
                error: "Token signature valid, but token string doesn't match database.".to_owned(),
            });
            response.status_code = 401;
            response
        }
        Err(VerifyTokenError::UserMismatch) => {
            let mut response = Response::json(&SingleErrorResponse {
                error: "Token user_id doesn't match database.".to_owned(),
            });
            response.status_code = 401;
            response
        }
        Err(VerifyTokenError::JwtError(error)) => match error.into_kind() {
            jwt::errors::ErrorKind::ExpiredSignature => {
                let mut response = Response::json(&SingleErrorResponse {
                    error: "Token has expired.".to_owned(),
                });
                response.status_code = 401;
                response
            }
            jwt::errors::ErrorKind::InvalidSignature => {
                let mut response = Response::json(&SingleErrorResponse {
                    error: "Token data is invalid/corrupted.".to_owned(),
                });
                response.status_code = 401;
                response
            }
            error => {
                let mut response = Response::json(&SingleErrorResponse {
                    error: format!("Unhandled jwt error: {:?}", error)
                        .to_owned(),
                });
                response.status_code = 500;
                response
            }
        },
        Err(error) => {
            let mut response = Response::json(&SingleErrorResponse {
                error: format!("Unhandled error: {:?}", error).to_owned(),
            });
            response.status_code = 500;
            response
        }
    }
}
