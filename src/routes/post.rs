use rocket::*;

#[get("/")]
pub fn post_list_rt() -> String {
    "List of posts".to_string()
}

#[post("/new")]
pub fn new_post_rt() -> String {
    "Creation of new post".to_string()
}

#[get("/<id>")]
pub fn info_post_rt(id: String) -> String {
    format!("Info for post {}", id)
}

#[put("/<id>")]
pub fn update_post_rt(id: String) -> String {
    format!("Update info for post {}", id)
}

#[delete("/<id>")]
pub fn delete_post_rt(id: String) -> String {
    format!("Delete post {}", id)
}
