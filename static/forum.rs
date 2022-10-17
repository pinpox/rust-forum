use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::forums;
use crate::db::establish_connection;

#[derive(Debug, Insertable)]
#[table_name = "forums"]
pub struct NewForum<'a> {
    pub name: &'a str,
    pub position: i32,
    pub is_locked: bool,
}

#[derive(Debug, Queryable, AsChangeset, Serialize, Deserialize)]
pub struct Forum {
    pub id: i32,
    pub name: String,
    pub position: i32,
    pub is_locked: bool,
}

pub fn create_forum(forum: NewForum) {

    println!("Creating forum: {:?}", forum);
    use crate::schema::forums::dsl::*;
    let mut connection = establish_connection();

    let new_forum = NewForum {
        name: &forum.name,
        position: forum.position,
        is_locked: forum.is_locked,
    };

    diesel::insert_into(forums)
        .values(&new_forum)
        .execute(&mut connection)
        .expect("Error saving new forum");
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

pub fn get_forums() -> Vec<Forum>{
    use crate::schema::forums::dsl::*;
    let mut connection = establish_connection();
    forums
        // .filter(removed.eq(false))
        .load::<Forum>(&mut connection)
        .expect("Error loading forums")
}
