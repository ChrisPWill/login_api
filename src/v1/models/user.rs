use chrono::{DateTime, Utc};
use validator::{Validate, ValidationError};

#[derive(Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(email(message = "Email is not valid"))]
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct CreateUserResponse {
    pub id: i64,
    pub email: String,
    pub date_created: DateTime<Utc>,
}

#[derive(Deserialize)]
pub enum PatchUserAction {
    ChangePassword,
}

#[derive(Deserialize, Validate)]
pub struct ChangePasswordData {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Deserialize, Validate)]
#[validate(schema(function = "validate_patch_user_request"))]
pub struct PatchUserRequest {
    pub action: PatchUserAction,
    pub change_password_data: Option<ChangePasswordData>,
}

fn validate_patch_user_request(
    request: &PatchUserRequest,
) -> Result<(), ValidationError> {
    match request.action {
        PatchUserAction::ChangePassword => match request.change_password_data {
            Some(_) => Ok(()),
            None => Err(ValidationError::new("change_password_data missing")),
        },
    }
}
