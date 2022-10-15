use rocket::*;

#[get("/")]
pub fn forum_list_rt() -> String {
    "List of forums".to_string()
}

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
