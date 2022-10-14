use diesel::prelude::*;
use chrono::{DateTime, Utc};


// use super::schema::posts;


#[derive(Queryable)]
pub struct Post {
    pub id: u64,
    pub user: u64,
    pub subject: String,
    pub content: String,
    pub topic_id: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// #[derive(Insertable)]
// #[diesel(table_name = posts)]
// pub struct NewPost<'a> {
//     pub subject: &'a str,
//     pub content: &'a str,
// }

