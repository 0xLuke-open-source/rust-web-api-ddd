use crate::domain::entities::User as DomainUser;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::infrastructure::database::schema::users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::infrastructure::database::schema::users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::infrastructure::database::schema::users)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password_hash: Option<String>,
    pub updated_at: NaiveDateTime,
}

impl From<User> for DomainUser {
    fn from(user: User) -> Self {
        DomainUser {
            id: user.id,
            name: user.name,
            email: user.email,
            password_hash: user.password_hash,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}
