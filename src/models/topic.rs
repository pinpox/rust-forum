use diesel::prelude::*;
use chrono::{DateTime, Utc};
#[derive(Queryable)]
pub struct Topic {
    pub id: u64,
    pub board_id: u64,
    pub is_sticky: bool,
    pub is_locked: bool,
    pub posts: u64,
    pub updated_at: DateTime<Utc>,
    // pub post: Post,
}