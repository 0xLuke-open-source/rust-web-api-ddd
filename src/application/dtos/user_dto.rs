use crate::User;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
pub struct CreateUserRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ValidateTokenRequest {
    pub token: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
            created_at: format!("{}", user.created_at.format("%Y-%m-%d %H:%M:%S")),
        }
    }
}
