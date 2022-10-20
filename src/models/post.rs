use crate::db::establish_connection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::topic::Topic;
use crate::schema::posts;

#[derive(Debug, Queryable, Identifiable, AsChangeset, Serialize, Deserialize, Associations)]
#[diesel(belongs_to(Topic))]
pub struct Post {
    pub id: i32,
    pub user_id: i32,
    pub content: String,
    pub topic_id: i32,
    pub created_at: i32,
}

#[derive(Debug, Insertable, FromForm)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub user_id: i32,
    pub content: String,
    pub topic_id: i32,
    #[field(default = 0)]
    pub created_at: i32,
}

pub fn create_post(post: NewPost) -> Result<usize, diesel::result::Error> {
    println!("Creating post: {:?}", post);
    use crate::schema::posts::dsl::*;
    let mut connection = establish_connection();

    let new_post = NewPost {
        user_id: post.user_id,
        content: post.content,
        topic_id: post.topic_id,
        created_at: post.created_at,
    };

    diesel::insert_into(posts)
        .values(&new_post)
        .execute(&mut connection)
}

// TODO Should we really return the raw diesel::result::Error here?
// Maybe wrap it in something?
pub fn get_posts() -> Result<Vec<Post>, diesel::result::Error> {
    use crate::schema::posts::dsl::*;
    let mut connection = establish_connection();
    posts
        // .filter(removed.eq(false))
        .load::<Post>(&mut connection)
}

pub fn get_post_by_id(f_id: i32) -> Result<Post, diesel::result::Error> {
    use crate::schema::posts::dsl::*;
    let mut connection = establish_connection();
    posts.find(f_id).first(&mut connection)
}

pub fn get_topic_posts(t_id: i32) -> Result<Vec<Post>, diesel::result::Error> {
    use crate::schema::posts::dsl::*;
    let mut connection = establish_connection();
    posts
        .filter(topic_id.eq(t_id))
        .load::<Post>(&mut connection)
}
