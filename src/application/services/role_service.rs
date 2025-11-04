use crate::application::dtos::role_dto::{CreateRoleRequest, RoleResponse};
use crate::domain::traits::RoleRepository;
use crate::infrastructure::database::models::role::Role;
use crate::shared::errors::error::Error;

pub struct RoleService<R: RoleRepository> {
    role_repository: R,
}

impl<R: RoleRepository> RoleService<R> {
    pub fn new(repository: R) -> Self {
        Self {
            role_repository: repository,
        }
    }

    pub async fn create_role(&self, request: CreateRoleRequest) -> Result<RoleResponse, Error> {
        let mut role = Role::new(request.id, request.name, request.description)?;
        self.role_repository.save(&mut role).await?;
        Ok(RoleResponse::from(role))
    }
}
