use rocket::*;

#[get("/")]
pub fn topic_list_rt() -> String {
    "List of topics".to_string()
}

#[post("/")]
pub fn new_topic_rt() -> String {
    "Creation of new topic".to_string()
}

#[get("/<id>")]
pub fn info_topic_rt(id: String) -> String {
    format!("Info for topic {}", id)
}

#[put("/<id>")]
pub fn update_topic_rt(id: String) -> String {
    format!("Update info for topic {}", id)
}

#[delete("/<id>")]
pub fn delete_topic_rt(id: String) -> String {
    format!("Delete topic {}", id)
}
