use diesel::prelude::*;

use serde::{Deserialize, Serialize};

use crate::db::establish_connection;
use crate::schema::users;

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

    //TODO!!!
    //Don't return all users, just the ones who's ID is one of:
    // - topic.user_id
    // - SELECT user_id FROM posts WHERE topic_id = topic.user_id

    // let distinct_user_ids = users.select(user_id).distinct()
    // .filter(removed.eq(false))
    // .load::<String>(&connection)?;

    users.load::<User>(&mut connection)
}
