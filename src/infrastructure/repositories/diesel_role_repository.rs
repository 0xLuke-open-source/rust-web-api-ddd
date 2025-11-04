use crate::domain::Repository;
use crate::domain::traits::RoleRepository;
use crate::infrastructure::database::DbPool;
use crate::infrastructure::database::models::role::{NewRole, Role};
use crate::infrastructure::database::schema::roles::dsl::roles;
use crate::infrastructure::repositories::DieselRepositoryBase;
use crate::shared::errors::error::Error;
use async_trait::async_trait;
use diesel::prelude::*;

#[derive(Clone)]
pub struct DieselRoleRepository {
    base: DieselRepositoryBase,
}

impl DieselRoleRepository {
    pub fn new(pool: DbPool) -> Self {
        Self {
            base: DieselRepositoryBase::new(pool),
        }
    }
}

#[async_trait]
impl Repository<Role, i64> for DieselRoleRepository {
    async fn find_by_id(&self, _id: i64) -> Result<Option<Role>, Error> {
        todo!()
    }

    async fn save(&self, role: &mut Role) -> Result<(), Error> {
        let mut conn = self.base.get_connection()?;
        let new_role = NewRole {
            id: role.id,
            name: role.name.clone(),
            description: role.description.clone(),
        };
        if role.id == 0 {
            diesel::insert_into(roles)
                .values(new_role)
                .execute(&mut conn)
                .map_err(|e| self.base.map_diesel_error(e))?;
        } else {
            diesel::update(roles.find(role.id))
                .set(&new_role)
                .execute(&mut conn)
                .map_err(|e| self.base.map_diesel_error(e))?;
        }
        Ok(())
    }

    async fn delete(&self, _id: i64) -> Result<(), Error> {
        todo!()
    }
}

#[async_trait]
impl RoleRepository for DieselRoleRepository {}
