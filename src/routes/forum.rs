use rocket::*;
use rocket_contrib::templates::Template;
use crate::models::forum::*;
use crate::models::board::*;
use std::collections::HashMap;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Serialize, Deserialize};



#[post("/")]
pub fn new_forum_rt() -> String {
    "Creation of new forum".to_string()
}

#[get("/<id>")]
pub fn info_forum_rt(id: String) -> String {
    format!("Info for forum {}", id)
}

#[put("/<id>")]
pub fn update_forum_rt(id: String) -> String {
    format!("Update info for forum {}", id)
}

#[delete("/<id>")]
pub fn delete_forum_rt(id: String) -> String {
    format!("Delete forum {}", id)
}



#[derive( Serialize, Deserialize)]
struct TemplateData {
    forums: Vec<Forum>,
    boards: Vec<Board>,
}

#[get("/")]
pub fn forum_list_rt() -> Template {


    let board1 = Board{
        id: 1,
        forum_id: 1,
        name: "Board 1".to_string(),
        description: "The first board".to_string(),
        updated_at: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(61, 0), Utc),
        position: 0,
        is_locked: false,
    };
    let board2 = Board{
        id: 2,
        forum_id: 1,
        name: "Board 2".to_string(),
        description: "The second board".to_string(),
        updated_at: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(62, 0), Utc),
        position: 0,
        is_locked: false,
    };

    let board3 = Board{
        id: 3,
        forum_id: 2,
        name: "Board 3".to_string(),
        description: "The third board".to_string(),
        updated_at: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(63, 0), Utc),
        position: 0,
        is_locked: false,
    };
    let board4 = Board{
        id: 4,
        forum_id: 2,
        name: "Board 4".to_string(),
        description: "The fourth board".to_string(),
        updated_at: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(63, 0), Utc),
        position: 0,
        is_locked: false,
    };


    let forum1 = Forum {
        id: 1,
        name: "General".to_string(),
        is_locked: false,
        position: 0,

    };

    let forum2 = Forum {
        id: 2,
        name: "Corrupted Memory".to_string(),
        is_locked: false,
        position: 1
    };


    let data = TemplateData{
     boards: vec![board1, board2, board3, board4],
    forums: vec![forum1, forum2]

    };


    // let forums = [forum1, forum2];
    // let mut context = HashMap::new();
    // context.insert("forums", &forums);
    // context.insert("boards", &boards);

    Template::render("forums", &data)
}

// #[get("/users")]
// pub fn user_list_rt() -> String {
//     "List of users".to_string()
// }

#[post("/")]
pub fn new_user_rt() -> String {
    "Creation of new user".to_string()
}

#[get("/<id>")]
pub fn info_user_rt(id: String) -> String {
    format!("Info for user {}", id)
}

#[post("/update/<id>")]
pub fn update_user_rt(id: String) -> String {
    format!("Update info for user {}", id)
}

#[get("/delete/<id>")]
pub fn delete_user_rt(id: String) -> String {
    format!("Delete user {}", id)
}
