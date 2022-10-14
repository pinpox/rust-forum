
use diesel::prelude::*;
use chrono::{DateTime, Utc};
#[derive(Queryable)]
pub struct Board {
    pub id: u64,
    pub forum_id: u64,
    pub name: String,
    pub description: String,
    pub topics: u64,
    pub posts: u64,
    pub updated_at: DateTime<Utc>,
    pub position: u64,
    pub is_locked: bool,
}
