use crate::models::board::*;
use crate::models::forum::*;
use rocket_dyn_templates::Template;
use serde_json::json;

use rocket::form::Form;

#[post("/", data = "<data>")]
pub fn create_board_rt(data: rocket::form::Result<Form<NewBoard>>) -> Template {
    let new_board = match data {
        Err(errors) => {
            let errs: Vec<String> = errors
                .iter()
                .map(|e| format!("{}", e.name.as_ref().expect(", ")))
                .collect();
            return Template::render(
                "board-new",
                json!({ "message": format!("Invalid values entered for: {}", errs.join(", ")) }),
            );
        }
        Ok(d) => NewBoard {
            updated_at: 0,
            name: d.name.to_string(),
            description: d.description.to_string(),
            forum_id: d.forum_id,
            position: d.position,
            is_locked: d.is_locked,
        },
    };

    match create_board(new_board) {
        Ok(_n) => render_new(format!("Board created")),
        Err(e) => render_new(format!("Failed to create board: {}", e)),
    }
}

fn render_new(message: String) -> Template {
    Template::render(
        "board-new",
        match get_forums() {
            Err(e) => json!({"message": e.to_string()}),
            Ok(f) => json!({
                "forums": f,
                "message": message,
            }),
        },
    )
}

#[get("/new")]
pub fn new_board_rt() -> Template {
    render_new("".to_string())
}

#[get("/<id>")]
pub fn info_board_rt(id: String) -> String {
    //TODO
    format!("Info for board {}", id)
}

#[put("/<id>")]
pub fn update_board_rt(id: String) -> String {
    //TODO
    format!("Update info for board {}", id)
}

#[delete("/<id>")]
pub fn delete_board_rt(id: String) -> String {
    //TODO
    format!("Delete board {}", id)
}
