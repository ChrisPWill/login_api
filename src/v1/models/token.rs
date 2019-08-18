use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct CreateTokenRequest {
    #[validate(email(message = "Email is not valid"))]
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct CreateTokenResponse {
    pub token: String,
}

#[derive(Deserialize, Validate)]
pub struct ValidateTokenRequest {
    pub token: String,
}

#[derive(Serialize)]
pub struct ValidateTokenResponse {
    pub user_id: i64,
    pub email: String,
}
