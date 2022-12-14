use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::db::establish_connection;
use crate::schema::forums;


#[derive(Debug, Queryable, AsChangeset, Serialize, Deserialize)]
pub struct Forum {
    pub id: i32,
    pub name: String,
    pub position: i32,
    pub is_locked: bool,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = forums)]
pub struct NewForum {
    pub name: String,
    pub position: i32,
    pub is_locked: bool,
}

#[derive(Debug, FromForm)]
pub struct NewForumRequest {
    #[field(validate = len(2..))]
    pub name: String,
    pub position: Option<String>,
    pub is_locked: bool,
}

pub fn create_forum(forum: NewForum) -> Result<usize, diesel::result::Error> {
    println!("Creating forum: {:?}", forum);
    use crate::schema::forums::dsl::*;
    let mut connection = establish_connection();

    let new_forum = NewForum {
        name: forum.name,
        position: forum.position,
        is_locked: forum.is_locked,
    };

    diesel::insert_into(forums)
        .values(&new_forum)
        .execute(&mut connection)

    // .expect("Error saving new forum");
}

pub fn update_forum(forum: Forum) {
    println!("Updating forum: {:?}", forum);
    use crate::schema::forums::dsl::*;
    let mut connection = establish_connection();

    diesel::update(forums.find(forum.id))
        .set(&forum)
        .execute(&mut connection)
        .expect("Error updating Forum");
}

pub fn get_forums() -> Result<Vec<Forum>, diesel::result::Error> {
    use crate::schema::forums::dsl::*;
    let mut connection = establish_connection();
    forums.load::<Forum>(&mut connection)
}

pub fn get_forum_by_id(f_id: i32) -> Result<Forum, diesel::result::Error> {
    use crate::schema::forums::dsl::*;
    let mut connection = establish_connection();
    forums.find(f_id).first(&mut connection)
}
