// infrastructure/repositories/diesel_menu_repository.rs
use crate::domain::{MenuRepository as DomainMenuRepository, Repository, entities::Menu};
use crate::infrastructure::database::models::menu::{NewMenu as DieselNewMenu, UpdateMenu};
use crate::infrastructure::{
    database::{DbPool, models::menu::Menu as DieselMenu, schema::menus::dsl::*},
    repositories::DieselRepositoryBase,
};
use async_trait::async_trait;
use diesel::prelude::*;
use crate::shared::errors::error::Error;

#[derive(Clone)]
pub struct DieselMenuRepository {
    base: DieselRepositoryBase,
}

impl DieselMenuRepository {
    pub fn new(pool: DbPool) -> Self {
        Self {
            base: DieselRepositoryBase::new(pool),
        }
    }
}
#[async_trait]
impl Repository<Menu, i64> for DieselMenuRepository {
    // 实现基本 CRUD 操作
    async fn find_by_id(&self, menu_id: i64) -> Result<Option<Menu>, Error> {
        let mut conn = self.base.get_connection()?;
        let result = menus
            .find(menu_id)
            .first::<DieselMenu>(&mut conn)
            .optional()
            .map_err(|e| self.base.map_diesel_error(e))?;
        Ok(result.map(|u| u.into()))
    }

    async fn save(&self, menu: &mut Menu) -> Result<(), Error> {
        let mut conn = self.base.get_connection()?;
        if menu.id == 0 {
            let new_menu = DieselNewMenu {
                name: menu.name.clone(),
                path: menu.path.clone(),
                component: menu.component.clone(),
                icon: menu.icon.clone(),
                parent_id: menu.parent_id,
                order_num: Option::from(menu.order_num),
            };
            diesel::insert_into(menus)
                .values(new_menu)
                .execute(&mut conn)
                .map_err(|e| self.base.map_diesel_error(e))?;
        } else {
            let new_menu = UpdateMenu {
                name: menu.name.clone(),
                path: menu.path.clone(),
                component: menu.component.clone(),
                icon: menu.icon.clone(),
                parent_id: menu.parent_id,
                order_num: Option::from(menu.order_num),
            };
            diesel::update(menus.find(menu.id))
                .set(&new_menu)
                .execute(&mut conn)
                .map_err(|e| self.base.map_diesel_error(e))?;
        }
        Ok(())
    }

    async fn delete(&self, menu_id: i64) -> Result<(), Error> {
        let mut conn = self.base.get_connection()?;
        let _ = diesel::delete(menus.find(menu_id))
            .execute(&mut conn)
            .map_err(|e| self.base.map_diesel_error(e))?;
        Ok(())
    }
}

#[async_trait]
impl DomainMenuRepository for DieselMenuRepository {
    // 实现菜单特定的业务方法
    async fn find_by_parent_id(&self, menu_parent_id: Option<i64>) -> Result<Vec<Menu>, Error> {
        let mut conn = self.base.get_connection()?;
        // 先转换为 BoxedSelectStatement
        let mut query = menus.into_boxed();
        if let Some(pid) = menu_parent_id {
            query = query.filter(parent_id.eq(pid));
        } else {
            query = query.filter(parent_id.is_null());
        }
        let results = query
            .load::<DieselMenu>(&mut conn)
            .map_err(|e| self.base.map_diesel_error(e))?;
        Ok(results.into_iter().map(|m| m.into()).collect())
    }

    async fn find_by_path(&self, _menu_path: &str) -> Result<Option<Menu>, Error> {
        todo!()
    }

    async fn find_children(&self, menu_parent_id: i64) -> Result<Vec<Menu>, Error> {
        let mut conn = self.base.get_connection()?;
        let results = menus
            .filter(parent_id.eq(menu_parent_id))
            .load::<DieselMenu>(&mut conn)
            .map_err(|e| self.base.map_diesel_error(e))?;
        Ok(results.into_iter().map(|m| m.into()).collect())
    }

    async fn get_all_menus(&self) -> Result<Vec<Menu>, Error> {
        let mut conn = self.base.get_connection()?;
        // 查询所有父级菜单和子级菜单,并且按照上下级关系返回
        let results = menus
            .load::<DieselMenu>(&mut conn)
            .map_err(|e| self.base.map_diesel_error(e))?;
        let all_menus: Vec<Menu> = results.into_iter().map(|m| m.into()).collect();
        // 构建菜单树
        let mut menu_tree: Vec<Menu> = Vec::new();
        // 找出所有顶级菜单 (parent_id 为 null)
        for menu in &all_menus {
            if menu.parent_id.is_none() {
                let mut top_menu = menu.clone();
                top_menu.children = self.build_menu_tree(menu.id, &all_menus).await?;
                menu_tree.push(top_menu);
            }
        }
        Ok(menu_tree)
    }

    // 递归构建菜单树
    async fn build_menu_tree(
        &self,
        parent_menu_id: i64,
        all_menus: &Vec<Menu>,
    ) -> Result<Vec<Menu>, Error> {
        let mut children: Vec<Menu> = Vec::new();

        // 找出当前父菜单的所有直接子菜单
        for menu in all_menus {
            if let Some(menu_parent_id) = menu.parent_id {
                if menu_parent_id == parent_menu_id {
                    let mut child_menu = menu.clone();
                    // 递归构建子菜单的子树
                    child_menu.children = self.build_menu_tree(menu.id, all_menus).await?;
                    children.push(child_menu);
                }
            }
        }

        Ok(children)
    }
}
