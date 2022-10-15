use rocket::*;

#[get("/")]
pub fn board_list_rt() -> String {
    "List of boards".to_string()
}

#[post("/")]
pub fn new_board_rt() -> String {
    "Creation of new board".to_string()
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
