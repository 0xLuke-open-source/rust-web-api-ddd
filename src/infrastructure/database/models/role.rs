use crate::shared::errors::error::Error;
use chrono::NaiveDateTime;
use diesel::Insertable;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::infrastructure::database::schema::roles)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Role {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
#[derive(Insertable, AsChangeset, Debug)]
#[diesel(table_name = crate::infrastructure::database::schema::roles)]
pub struct NewRole {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
}
impl Role {
    pub fn new(id: i64, name: String, description: Option<String>) -> Result<Self, Error> {
        Ok(Role {
            id,
            name,
            description,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        })
    }
}

//测试
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn test_new() {
        // let role = Role::new("admin".to_string(), Some("管理员".to_string()));
        // println!("{:?}", json!(role).to_string());
    }
}
