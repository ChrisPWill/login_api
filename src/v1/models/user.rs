use chrono::{DateTime, Utc};
use validator::Validate;

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
