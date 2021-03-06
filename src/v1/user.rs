use dal::{users::CreateUserError, DalConnection};
use handlers;
use rouille::{
    input::{json::JsonError, json_input},
    Request,
    Response,
};
use v1::models::{
    response::SingleErrorResponse,
    user::{CreateUserRequest, CreateUserResponse, PatchUserRequest},
};
use validator::Validate;

pub fn routes(request: &Request, connection: &DalConnection) -> Response {
    router!(
        request,
        (POST) [""] => create_user(request, connection),
        (PATCH) ["/{user_id}", user_id: i64] => patch_user(request, connection, user_id),
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

fn patch_user(
    request: &Request,
    connection: &DalConnection,
    user_id: i64,
) -> Response {
    let body: PatchUserRequest = match json_input(request) {
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

    let mut response = Response::json(&SingleErrorResponse {
        error: "Not implemented".to_owned(),
    });
    response.status_code = 501;
    response
}
