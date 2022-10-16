use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Forum {
    pub id: u64,
    pub name: String,
    pub position: u64,
    pub is_locked: bool,
}
