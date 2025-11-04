use crate::User;
use crate::domain::entities::menu::Menu;
use async_trait::async_trait;
use crate::infrastructure::database::models::role::Role;
use crate::shared::errors::error::Error;
#[async_trait]
pub trait Repository<T, ID>: Send + Sync {
    async fn find_by_id(&self, id: ID) -> Result<Option<T>, Error>;
    async fn save(&self, entity: &mut T) -> Result<(), Error>;
    async fn delete(&self, id: ID) -> Result<(), Error>;
}

#[async_trait]
pub trait UserRepository: Repository<User, i64> {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, Error>;

    async fn find_all(&self) -> Result<Vec<User>, Error>;
}

#[async_trait]
pub trait MenuRepository: Repository<Menu, i64> {
    async fn find_by_parent_id(&self, menu_parent_id: Option<i64>) -> Result<Vec<Menu>, Error>;
    async fn find_by_path(&self, menu_path: &str) -> Result<Option<Menu>, Error>;
    async fn find_children(&self, menu_parent_id: i64) -> Result<Vec<Menu>, Error>;

    async fn get_all_menus(&self) -> Result<Vec<Menu>, Error>; // 递归构建菜单树
    async fn build_menu_tree(&self, parent_id: i64, all_menus: &Vec<Menu>) -> Result<Vec<Menu>, Error>;
}

#[async_trait]
pub trait RoleRepository: Repository<Role, i64> {


}