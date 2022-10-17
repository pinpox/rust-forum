
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


#[derive(Queryable, Serialize, Deserialize)]
pub struct NewBoard {
    pub forum_id: u64,
    pub name: String,
    pub description: String,
    pub position: u64,
    pub is_locked: bool,
}


pub fn create_board(forum: NewBoard) {

    println!("Creating forum: {:?}", forum);
    use crate::schema::boards::dsl::*;
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

pub fn update_board(forum: Forum) {
    println!("Updating forum: {:?}", forum);
    use crate::schema::forums::dsl::*;
    let mut connection = establish_connection();


    diesel::update(forums.find(forum.id))
        .set(&forum)
        .execute(&mut connection)
        .expect("Error updating Forum");
}

pub fn get_boards() -> Vec<Forum>{
    use crate::schema::forums::dsl::*;
    let mut connection = establish_connection();
    forums
        // .filter(removed.eq(false))
        .load::<Forum>(&mut connection)
        .expect("Error loading forums")
}
