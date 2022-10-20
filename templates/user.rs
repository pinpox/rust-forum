use diesel::prelude::*;

use serde::{Serialize, Deserialize};

use crate::schema::users;
use crate::db::establish_connection;

#[derive(Debug, Queryable, Identifiable, AsChangeset, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub about: String,
    pub picture: String,
    pub password: String,
    pub is_admin: bool,
}

// Selects all users that participate in a topic (creator + repliers)
pub fn get_topic_users(t_id: i32) -> Result<Vec<User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    let mut connection = establish_connection();


    // ID is one of:
    // - topic.user_id
    // - SELECT user_id FROM posts WHERE topic_id = topic.user_id


// let distinct_user_ids = users.select(user_id).distinct()
        // .filter(removed.eq(false))
    // .load::<String>(&connection)?;


    users
        // .filter(topic_id.eq(t_id))
        .load::<User>(&mut connection)
}
