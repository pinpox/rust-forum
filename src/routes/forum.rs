use crate::models::board::*;
use crate::models::forum::*;
use crate::models::topic::*;
use rocket::*;
use rocket_contrib::templates::Template;
use serde::{Deserialize, Serialize};

use rocket::request::Form;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct TemplateData {
    forums: Vec<Forum>,
    boards: Vec<Board>,
    topics: Vec<Topic>,
    message: Option<String>,
}

//TODO show error on invalid inputs
#[post("/", data = "<data>")]
pub fn create_forum_rt(data: Form<NewForum>) -> Template {
    let new_forum = NewForum {
        name: data.name.to_string(),
        position: data.position,
        is_locked: data.is_locked,
    };
    let mut context: HashMap<String, String> = HashMap::new();

    // TODO show flash message instead of raw error
    match create_forum(new_forum) {
        Ok(_n) => context.insert("message".to_string(), format!("Forum created")),
        Err(e) => context.insert("message".to_string(), format!("Failed to create forum: {}", e ))
    };

    Template::render("forum-new", context)
}

#[get("/new")]
pub fn new_forum_rt() -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("forum-new", context)
}

#[get("/<id>")]
pub fn info_forum_rt(id: i32) -> Template {
    let mut data = TemplateData {
        message: None,
        boards: vec![],
        topics: vec![],
        forums: vec![],
    };

    match get_forum_by_id(id) {
        Err(e) => data.message = Some(e.to_string()),
        Ok(f) => match get_forum_boards(f.id) {
            Err(e) => data.message = Some(e.to_string()),
            Ok(b) => {
                data = TemplateData {
                    boards: b,
                    forums: vec![f],
                    topics: vec![],
                    message: None,
                };
            }
        },
    };

    Template::render("forum", &data)
}

#[put("/<id>")]
pub fn update_forum_rt(id: String) -> String {
    format!("Update info for forum {}", id)
}

#[delete("/<id>")]
pub fn delete_forum_rt(id: String) -> String {
    format!("Delete forum {}", id)
}

#[get("/")]
pub fn forum_list_rt() -> Template {
    let mut data = TemplateData {
        message: None,
        boards: vec![],
        topics: vec![],
        forums: vec![],
    };

    match get_forums() {
        Err(e) => data.message = Some(e.to_string()),
        Ok(f) => match get_boards() {
            Err(e) => data.message = Some(e.to_string()),
            Ok(b) => {
                data.boards = b;
                data.forums = f;
            }
        },
    }
    Template::render("forums", &data)
}
