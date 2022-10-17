use diesel::prelude::*;
use serde::{Serialize, Deserialize};




// use crate::db::establish_connection;
use crate::schema::topics;
use crate::models::board::Board;

#[derive(Debug, Queryable, Identifiable, AsChangeset, Serialize, Deserialize, Associations)]
#[diesel(belongs_to(Board))]
pub struct Topic {
    pub id: i32,
    pub board_id: i32,
    pub is_sticky: bool,
    pub is_locked: bool,
    pub created_at: chrono::NaiveDateTime,
    // pub post: Post,
}
