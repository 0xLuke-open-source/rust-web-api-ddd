use chrono::{NaiveDateTime, Utc};
// src/infrastructure/database/models/menu.rs
use crate::domain::entities::Menu as DomainMenu;
use diesel::prelude::*;
use crate::shared::errors::error::Error;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::infrastructure::database::schema::menus)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Menu {
    pub id: i64,
    pub name: String,
    pub path: String,
    pub component: String,
    pub icon: Option<String>,
    pub parent_id: Option<i64>,
    pub order_num: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::infrastructure::database::schema::menus)]
pub struct NewMenu {
    pub name: String,
    pub path: String,
    pub component: String,
    pub icon: Option<String>,
    pub parent_id: Option<i64>,
    pub order_num: Option<i32>,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::infrastructure::database::schema::menus)]
pub struct UpdateMenu {
    pub name: String,
    pub path: String,
    pub component: String,
    pub icon: Option<String>,
    pub parent_id: Option<i64>,
    pub order_num: Option<i32>,
}

impl Menu {
    pub fn new(
        name: String,
        path: String,
        component: String,
        icon: Option<String>,
        parent_id: Option<i64>,
        order_num: Option<i32>,
    ) -> Result<Self, Error> {
        let now = Utc::now().naive_utc();
        Ok(Menu {
            id: 0,
            name,
            path,
            component,
            icon,
            parent_id,
            order_num,
            created_at: now,
            updated_at: now,
        })
    }
}

impl From<Menu> for DomainMenu {
    fn from(menu: Menu) -> Self {
        DomainMenu {
            id: menu.id,
            name: menu.name,
            path: menu.path,
            component: menu.component,
            icon: menu.icon,
            parent_id: menu.parent_id,
            order_num: menu.order_num,
            created_at: menu.created_at,
            updated_at: menu.updated_at,
            children: vec![],
        }
    }
}
