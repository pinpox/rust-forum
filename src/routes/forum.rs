use crate::models::board::*;
use crate::models::forum::*;
use crate::models::topic::*;
// use rocket::*;
use rocket_dyn_templates::Template;
use serde::{Deserialize, Serialize};

use rocket::form::Form;

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


    let mut data = TemplateData {
        message: None,
        boards: vec![],
        topics: vec![],
        forums: vec![],
    };

    // TODO show flash message instead of raw error
    match create_forum(new_forum) {
        Ok(_n) => data.message = Some("Forum created".to_string()),
        Err(e) => data.message = Some(format!("Error creating forum: {}", e)),
    };

    Template::render("forum-new", data)
}

#[get("/new")]
pub fn new_forum_rt() -> Template {
    let data = TemplateData {
        message: None,
        boards: vec![],
        topics: vec![],
        forums: vec![],
    };
    Template::render("forum-new", data)
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
                println!("{:#?}", b);
                data.boards = b;
                data.forums = f;
            }
        },
    }
    Template::render("forums", &data)
}
