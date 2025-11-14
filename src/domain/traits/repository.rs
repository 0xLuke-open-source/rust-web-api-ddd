use async_trait::async_trait;
use crate::shared::errors::error::Error;

#[async_trait]
pub trait Repository<T, ID>: Send + Sync {
    async fn find_by_id(&self, id: ID) -> Result<Option<T>, Error>;
    async fn save(&self, entity: &mut T) -> Result<(), Error>;
    async fn delete(&self, id: ID) -> Result<(), Error>;
}