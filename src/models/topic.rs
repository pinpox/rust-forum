use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::db::establish_connection;

use crate::models::board::Board;
use crate::schema::topics;

#[derive(Debug, Queryable, Identifiable, AsChangeset, Serialize, Deserialize, Associations)]
#[diesel(belongs_to(Board))]
pub struct Topic {
    pub id: i32,
    pub subject: String,
    pub board_id: i32,
    pub is_sticky: bool,
    pub is_locked: bool,
    pub created_at: i32,
}


#[derive(Debug, Insertable, FromForm)]
#[diesel(table_name = topics)]
pub struct NewTopic {
    pub subject: String,
    pub board_id: i32,
    #[field(default = false)]
    pub is_locked: bool,
    #[field(default = false)]
    pub is_sticky: bool,
}



pub fn create_topic(topic: NewTopic) -> Result<usize, diesel::result::Error> {
    println!("Creating topic: {:?}", topic);
    use crate::schema::topics::dsl::*;
    let mut connection = establish_connection();


    let new_topic = NewTopic {
        subject: topic.subject,
        board_id: topic.board_id,
        is_locked: topic.is_locked,
        is_sticky: topic.is_sticky,
    };

    diesel::insert_into(topics)
        .values(&new_topic)
        .execute(&mut connection)
}



pub fn get_topic_by_id(f_id: i32) -> Result<Topic, diesel::result::Error> {
    use crate::schema::topics::dsl::*;
    let mut connection = establish_connection();
    topics.find(f_id).first(&mut connection)
}

// pub fn get_board_topics(b_id: i32) -> Result<Vec<Topic>, diesel::result::Error> {
//     use crate::schema::topics::dsl::*;
//     let mut connection = establish_connection();
//     topics
//         .filter(board_id.eq(b_id))
//         .load::<Topic>(&mut connection)
// }
