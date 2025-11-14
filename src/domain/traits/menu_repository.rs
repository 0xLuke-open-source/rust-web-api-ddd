use async_trait::async_trait;
use crate::domain::entities::Menu;
use crate::domain::traits::repository::Repository;

use crate::shared::errors::error::Error;

#[async_trait]
pub trait MenuRepository: Repository<Menu, i64> {
    async fn find_by_parent_id(&self, menu_parent_id: Option<i64>) -> Result<Vec<Menu>, Error>;
    async fn find_by_path(&self, menu_path: &str) -> Result<Option<Menu>, Error>;
    async fn find_children(&self, menu_parent_id: i64) -> Result<Vec<Menu>, Error>;

    async fn get_all_menus(&self) -> Result<Vec<Menu>, Error>; // 递归构建菜单树
    async fn build_menu_tree(&self, parent_id: i64, all_menus: &Vec<Menu>) -> Result<Vec<Menu>, Error>;
}