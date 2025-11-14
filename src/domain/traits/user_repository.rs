use async_trait::async_trait;
use crate::domain::traits::repository::Repository;
use crate::shared::errors::error::Error;
use crate::User;

#[async_trait]
pub trait UserRepository: Repository<User, i64> {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, Error>;

    async fn find_all(&self) -> Result<Vec<User>, Error>;
}
