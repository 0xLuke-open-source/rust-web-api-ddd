use crate::infrastructure::database::models::role::Role;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateRoleRequest {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct RoleResponse {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Role> for RoleResponse {
    fn from(role: Role) -> Self {
        Self {
            id: role.id,
            name: role.name,
            description: role.description,
            created_at: format!("{}", role.created_at.format("%Y-%m-%d %H:%M:%S")),
            updated_at: format!("{}", role.updated_at.format("%Y-%m-%d %H:%M:%S")),
        }
    }
}
