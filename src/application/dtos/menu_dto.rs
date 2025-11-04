use crate::domain::entities::Menu;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
pub struct CreateMenuRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub path: String,
    pub component: String,
    pub icon: String,
    pub parent_id: i64,
    pub order_num: i32,
}

#[derive(Debug, Serialize)]
pub struct MenuResponse {
    pub id: i64,
    pub name: String,
    pub path: String,
    pub component: String,
    pub icon: Option<String>,
    pub parent_id: Option<i64>,
    pub order_num: Option<i32>,
    pub create_at: String,
    pub update_at: String,
    pub children: Vec<MenuResponse>,
}

impl From<Menu> for MenuResponse {
    fn from(menu: Menu) -> Self {
        Self {
            id: menu.id,
            name: menu.name,
            path: menu.path,
            component: menu.component,
            icon: menu.icon,
            parent_id: menu.parent_id,
            order_num: menu.order_num,
            create_at: format!("{}", menu.created_at.format("%Y-%m-%d %H:%M:%S")),
            update_at: format!("{}", menu.updated_at.format("%Y-%m-%d %H:%M:%S")),
            children: menu.children.into_iter().map(|m| m.into()).collect(),
        }
    }
}

#[derive(Debug, Validate, Deserialize)]
pub struct UpdateMenuDTO {
    pub id: i64,
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub path: String,
    pub component: String,
    pub icon: String,
    pub parent_id: i64,
    pub order_num: i32,
}

#[derive(Debug, Validate, Deserialize)]
pub struct DeleteMenuDTO {
    pub id: i64,
}

#[derive(Debug, Validate, Deserialize)]
pub struct QueryMenuDTO {
    pub id: i64,
}

#[derive(Debug, Validate, Deserialize)]
pub struct QueryMenuListDTO {
    pub parent_id: i64,
}

#[derive(Debug, Validate, Deserialize)]
pub struct QueryMenuTreeDTO {
    pub parent_id: i64,
}

#[derive(Debug, Validate, Deserialize)]
pub struct QueryMenuTreeListDTO {
    pub parent_id: i64,
}

#[derive(Debug, Validate, Deserialize)]
pub struct QueryMenuTreeListByRoleIdDTO {
    pub role_id: i64,
}
