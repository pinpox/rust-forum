use diesel::prelude::*;
#[derive(Queryable)]
pub struct Forum {
    pub id: u64,
    pub name: String,
    pub position: u64,
    pub is_locked: bool,
}
