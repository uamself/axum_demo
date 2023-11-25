use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub password: String,
    pub enabled: i32,
    pub create_time: NaiveDateTime,
    pub modify_time: NaiveDateTime,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub create_time: NaiveDateTime,
    pub modify_time: NaiveDateTime,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct UserRole {
    pub id: String,
    pub user_id: String,
    pub role_id: String,
    pub create_time: NaiveDateTime,
    pub modify_time: NaiveDateTime,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct Permissions {
    pub id: String,
    pub name: String,
    pub create_time: NaiveDateTime,
    pub modify_time: NaiveDateTime,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct RolePermissions {
    pub id: String,
    pub role_id: String,
    pub permissions_id: String,
    pub create_time: NaiveDateTime,
    pub modify_time: NaiveDateTime,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct Menu {
    pub id: String,
    pub name: String,
    pub permissions_id: String,
    pub url: String,
    pub sort: i32,
    pub style: String,
    pub parent_id: String,
    pub create_time: NaiveDateTime,
    pub modify_time: NaiveDateTime,
}
