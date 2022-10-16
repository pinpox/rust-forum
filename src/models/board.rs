
use diesel::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
#[derive(Queryable, Serialize, Deserialize)]
pub struct Board {
    pub id: u64,
    pub forum_id: u64,
    pub name: String,
    pub description: String,
    pub updated_at: DateTime<Utc>,
    pub position: u64,
    pub is_locked: bool,
}
