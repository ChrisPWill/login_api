use chrono::{DateTime, Utc};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct CreateUserResponse {
    pub id: i32,
    pub email: String,
    pub date_created: DateTime<Utc>,
}
