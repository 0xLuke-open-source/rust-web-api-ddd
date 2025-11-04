use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use crate::shared::errors::error::Error;

// domain/entities/menu.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub children: Vec<Menu>
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
        Ok(Self {
            id: 0,
            name,
            path,
            component,
            icon,
            parent_id,
            order_num,
            created_at: Default::default(),
            updated_at: Default::default(),
            children: vec![],
        })
    }
}
