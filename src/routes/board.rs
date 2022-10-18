use crate::models::board::*;
use crate::models::forum::*;
use rocket_contrib::templates::Template;
use serde::{Deserialize, Serialize};

use rocket::request::Form;
use rocket::*;

#[derive(Serialize, Deserialize)]
struct TemplateData {
    forums: Vec<Forum>,
    boards: Vec<Board>,
    // topics: Vec<Topic>,
    message: Option<String>,
}



//TODO show error on invalid inputs
#[post("/", data = "<data>")]
pub fn create_board_rt(data: Form<NewBoard>) -> Template {

    // let test = data.into_inner();
    println!("THE DATA {:#?}", data);

    let new_board = NewBoard {
        updated_at: 0,
        name: data.name.to_string(),
        description: data.description.to_string(),
        forum_id: data.forum_id,
        position: data.position,
        is_locked: data.is_locked,
    };

    let mut data = TemplateData {
        message: None,
        boards: vec![],
        // topics: vec![],
        forums: vec![],
    };

    // TODO show flash message instead of raw error
    match create_board(new_board) {
        Ok(_n) => data.message = Some(format!("Board created")),
        Err(e) => data.message = Some(format!("Failed to create board: {}", e)),
    };

    Template::render("board-new", data)
}

#[get("/new")]
pub fn new_board_rt() -> Template {
    let mut data = TemplateData {
        message: None,
        boards: vec![],
        // topics: vec![],
        forums: vec![],
    };

    match get_forums() {
        Err(e) => data.message = Some(e.to_string()),
        Ok(f) => {
            data.forums = f;
        }
    };

    Template::render("board-new", data)
}

#[get("/")]
pub fn board_list_rt() -> String {
    "List of boards".to_string()
}

#[get("/<id>")]
pub fn info_board_rt(id: String) -> String {
    format!("Info for board {}", id)
}

#[put("/<id>")]
pub fn update_board_rt(id: String) -> String {
    format!("Update info for board {}", id)
}

#[delete("/<id>")]
pub fn delete_board_rt(id: String) -> String {
    format!("Delete board {}", id)
}
