use diesel::prelude::*;
#[derive(Queryable)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub password: String,
    pub is_admin: bool,
    pub about: String,
    pub picture: String,
}
