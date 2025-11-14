use async_trait::async_trait;
use diesel::prelude::*;
use crate::domain::traits::{
    UserRepository as DomainUserRepository,
    Repository
};
use crate::shared::errors::error::Error;
use crate::infrastructure::database::schema::users::dsl::*;
use crate::infrastructure::database::models::user::{User as DieselUser, NewUser as DieselNewUser, UpdateUser};
use crate::domain::entities::User;
use crate::infrastructure::database::DbPool;
use crate::infrastructure::repositories::DieselRepositoryBase;

#[derive(Clone)]
pub struct DieselUserRepository {
    base: DieselRepositoryBase,
}
impl DieselUserRepository {
    pub fn new(pool: DbPool) -> Self {
        Self {
            base: DieselRepositoryBase::new(pool)
        }
    }
}

// 实现 Repository trait
#[async_trait]
impl Repository<User, i64> for DieselUserRepository {
    async fn find_by_id(&self, user_id: i64) -> Result<Option<User>, Error> {
        let mut conn = self.base.get_connection()?;
        let result = users
            .find(user_id)
            .first::<DieselUser>(&mut conn)
            .optional()
            .map_err(|e| self.base.map_diesel_error(e))?;

        Ok(result.map(|u| u.into()))
    }

    async fn save(&self, user: &mut User) -> Result<(), Error> {
        let mut conn = self.base.get_connection()?;

        if user.id == 0 {
            // 插入新用户
            let new_user = DieselNewUser {
                name: user.name.clone(),
                email: user.email.clone(),
                password_hash: user.password_hash.clone(),
                created_at: user.created_at,
                updated_at: user.updated_at,
            };

            // 使用 execute 执行插入
            diesel::insert_into(users)
                .values(&new_user)
                .execute(&mut conn)
                .map_err(|e| self.base.map_diesel_error(e))?;

            // 获取最后插入的 ID
            let last_id: i64 = diesel::select(diesel::dsl::sql::<diesel::sql_types::BigInt>("LAST_INSERT_ID()"))
                .get_result(&mut conn)
                .map_err(|e| self.base.map_diesel_error(e))?;

            user.id = last_id;
        } else {
            // 更新用户
            let update_user = UpdateUser {
                name: Some(user.name.clone()),
                email: Some(user.email.clone()),
                password_hash: Some(user.password_hash.clone()),
                updated_at: user.updated_at,
            };

            diesel::update(users.find(user.id))
                .set(&update_user)
                .execute(&mut conn)
                .map_err(|e| self.base.map_diesel_error(e))?;
        }

        Ok(())
    }

    async fn delete(&self, user_id: i64) -> Result<(), Error> {
        let mut conn = self.base.get_connection()?;
        diesel::delete(users.find(user_id))
            .execute(&mut conn)
            .map_err(|e| self.base.map_diesel_error(e))?;

        Ok(())
    }
}

// 实现 UserRepository trait
#[async_trait]
impl DomainUserRepository for DieselUserRepository {
    async fn find_by_email(&self, email_addr: &str) -> Result<Option<User>, Error> {
        let mut conn = self.base.get_connection()?;
        let result = users
            .filter(email.eq(email_addr))
            .first::<DieselUser>(&mut conn)
            .optional()
            .map_err(|e| self.base.map_diesel_error(e))?;

        Ok(result.map(|u| u.into()))
    }

    async fn find_all(&self) -> Result<Vec<User>, Error> {
        let mut conn = self.base.get_connection()?;
        let result = users
            .order_by(id.desc())
            .load::<DieselUser>(&mut conn)
            .map_err(|e| self.base.map_diesel_error(e))?;
            Ok(result.into_iter().map(|u| u.into()).collect())
    }
}