use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::shared::errors::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Validate, Deserialize)]
pub struct NewUser {
    #[validate(length(min = 1, max = 100))]
    pub name: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct UpdateUser {
    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,

    #[validate(email)]
    pub email: Option<String>,
}

impl User {
    pub fn new(name: String, email: String, password_hash: String) -> Result<Self, Error> {
        let now = Utc::now().naive_utc();

        Ok(Self {
            id: 0,
            name,
            email,
            password_hash,
            created_at: now,
            updated_at: now,
        })
    }

    pub fn update_name(&mut self, name: String) -> Result<(), Error> {
        if name.is_empty() || name.len() > 100 {
            return Err(Error::Validation("Invalid name length".into()));
        }

        self.name = name;
        self.updated_at = Utc::now().naive_utc();
        Ok(())
    }

    pub fn update_email(&mut self, email: String) -> Result<(), Error> {
        // if !validator::validate_email(&email) {
        //     return Err(crate::shared::error::Error::Validation("Invalid email format".into()));
        // }

        self.email = email;
        self.updated_at = Utc::now().naive_utc();
        Ok(())
    }
}