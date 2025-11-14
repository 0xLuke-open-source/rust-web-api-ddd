use async_trait::async_trait;
use crate::domain::traits::repository::Repository;
use crate::infrastructure::database::models::role::Role;

#[async_trait]
pub trait RoleRepository: Repository<Role, i64> {


}