use crate::application::dtos::user_dto::{
    CreateUserRequest, LoginRequest, LoginResponse, UserResponse,
};
use crate::domain::{entities::UpdateUser, entities::User, traits::UserRepository};
use crate::shared::errors::error::Error;
use argon2::password_hash::{SaltString, rand_core::OsRng};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use validator::ValidateEmail;

#[derive(Clone)]
pub struct UserService<R: UserRepository> {
    user_repository: Arc<R>,
}

impl<R: UserRepository> UserService<R> {
    pub fn new(user_repository: R) -> Self {
        Self {
            user_repository: Arc::new(user_repository),
        }
    }

    pub async fn create_user(&self, request: CreateUserRequest) -> Result<UserResponse, Error> {
        if request.password.is_empty() {
            return Err(Error::Validation("Passwords do not match".into()));
        }
        if request.name.is_empty() {
            return Err(Error::Validation("Name is empty".into()));
        }
        //验证邮箱合法性
        if !request.email.validate_email() {
            return Err(Error::Validation("Invalid email".into()));
        }
        // 检查邮箱是否已存在
        if self
            .user_repository
            .find_by_email(&request.email)
            .await?
            .is_some()
        {
            return Err(Error::Conflict(
                "User with this email already exists".into(),
            ));
        }

        // 哈希密码
        let password_hash = self.hash_password(&request.password)?;

        // 创建用户
        let mut user = User::new(request.name, request.email, password_hash)?;

        // 保存到数据库
        self.user_repository.save(&mut user).await?;

        Ok(UserResponse::from(user))
    }

    pub async fn get_user(&self, user_id: i64) -> Result<UserResponse, Error> {
        let user = self
            .user_repository
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| Error::NotFound("User not found".into()))?;

        Ok(UserResponse::from(user))
    }

    pub async fn update_user(
        &self,
        user_id: i64,
        update_data: UpdateUser,
    ) -> Result<UserResponse, Error> {
        let mut user = self
            .user_repository
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| Error::NotFound("User not found".into()))?;

        // 更新字段
        if let Some(name) = update_data.name {
            user.update_name(name)?;
        }
        if let Some(email) = update_data.email {
            user.update_email(email)?;
        }

        // 保存更新
        self.user_repository.save(&mut user).await?;

        Ok(UserResponse::from(user))
    }

    pub async fn delete_user(&self, user_id: i64) -> Result<(), Error> {
        self.user_repository.delete(user_id).await
    }

    fn hash_password(&self, password: &str) -> Result<String, Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)?
            .to_string();

        Ok(password_hash)
    }

    pub async fn get_all_users(&self) -> Result<Vec<UserResponse>, Error> {
        let users = self.user_repository.find_all().await?;
        Ok(users.into_iter().map(UserResponse::from).collect())
    }

    pub async fn login(&self, request: LoginRequest) -> Result<LoginResponse, Error> {
        if request.password.is_empty() {
            return Err(Error::Validation("Passwords do not match".into()));
        }
        if request.email.is_empty() {
            return Err(Error::Validation("Email is empty".into()));
        }
        if !request.email.validate_email() {
            return Err(Error::Validation("Invalid email".into()));
        }
        let user = self
            .user_repository
            .find_by_email(&request.email)
            .await?
            .ok_or_else(|| Error::NotFound("User not found".into()))?;

        // 验证密码
        let argon2 = Argon2::default();
        let password_hash = PasswordHash::new(&user.password_hash)?;
        if argon2
            .verify_password(request.password.as_bytes(), &password_hash)
            .is_err()
        {
            return Err(Error::Unauthorized("Invalid password".into()));
        }
        let token = generate_token(&user)?;
        Ok(LoginResponse {
            token,
            user: UserResponse::from(user),
        })
    }
}
fn generate_token(user: &User) -> Result<String, Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();
    let claims = Claims {
        sub: user.id,
        email: user.email.clone(),
        exp: expiration as usize,
    };

    let header = Header::default();
    let encoding_key = EncodingKey::from_secret(user.id.to_string().as_bytes());
    let token = jsonwebtoken::encode(&header, &claims, &encoding_key)
        .map_err(|_| Error::InvalidToken("Failed to generate token".into()))?;
    Ok(token)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: i64,
    email: String,
    exp: usize,
}


#[cfg(test)]
mod tests {}
