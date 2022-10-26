use crate::models::board::*;
use crate::models::forum::*;
use crate::models::topic::*;
use crate::models::user::User;

use rocket::form::Form;
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket_dyn_templates::Template;
use serde_json::json;

#[post("/", data = "<data>")]
pub fn create_board_rt(
    data: rocket::form::Result<Form<NewBoardRequest>>,
    _user: User,
) -> Flash<Redirect> {
    let new_board = match data {
        Err(errors) => {
            let errs: Vec<String> = errors
                .iter()
                .map(|e| format!("{}", e.name.as_ref().expect(", ")))
                .collect();
            return Flash::error(
                Redirect::to(uri!("/boards", new_board_rt)),
                format!(
                    "Error creating board. Invalid values entered for: {}",
                    errs.join(", ")
                ),
            );
        }

        Ok(d) => NewBoard {
            updated_at: 0, //TODO
            name: d.name.to_string(),
            description: d.description.to_string(),
            forum_id: d.forum_id,
            position: d
                .position
                .to_owned()
                .unwrap_or_default()
                .parse::<i32>()
                .unwrap_or_default(),
            is_locked: d.is_locked,
        },
    };

    match create_board(new_board) {
        Ok(n) => Flash::success(
            Redirect::to(uri!("/boards", new_board_rt)),
            format!("Sucessfully created board: {}", n),
        ),
        Err(e) => Flash::error(
            Redirect::to(uri!("/boards", new_board_rt)),
            format!("Error creating board: {}", e),
        ),
    }
}

#[get("/new")]
pub fn new_board_rt(flash: Option<FlashMessage>) -> Template {
    Template::render(
        "board-new",
        match get_forums() {
            Err(e) => json!({"flash": e.to_string()}),
            Ok(f) => json!({
                "forums": f,
                "flash": flash,
            }),
        },
    )
}

#[get("/<id>")]
pub fn info_board_rt(id: i32, user: Option<User>) -> Template {
    Template::render(
        "board",
        match get_board_by_id(id) {
            Err(e) => json!({"flash": e.to_string()}),
            Ok(b) => match get_board_topics(b.id) {
                Err(e) => json!({"flash": e.to_string()}),
                Ok(t) => {
                    json!({
                        "topics": t,
                        "board": b,
                        "user": user
                    })
                }
            },
        },
    )
}

//#[put("/<id>")]
//pub fn update_board_rt(id: String) -> String {
//    //TODO
//    format!("Update info for board {}", id)
//}

//#[delete("/<id>")]
//pub fn delete_board_rt(id: String) -> String {
//    //TODO
//    format!("Delete board {}", id)
//}
