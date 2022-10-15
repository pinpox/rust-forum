use rocket::*;

use rocket_contrib::templates::Template;

use crate::models::user::*;
use std::collections::HashMap;

#[get("/")]
pub fn user_list_rt() -> Template {
    let user2 = User {
        id: 2,
        about: "About user 2".to_string(),
        name: "Username2".to_string(),
        password: "pw2".to_string(),
        picture: "pic2".to_string(),
        is_admin: false,
    };

    let user1 = User {
        id: 1,
        about: "About user 1".to_string(),
        name: "Username1".to_string(),
        password: "pw1".to_string(),
        picture: "pic1".to_string(),
        is_admin:true,
    };

    let users = [user1, user2];
    let mut context = HashMap::new();
    context.insert("users", &users);

    // let context = nil context();
    // Template::render("user-list.tera", &context)
    Template::render("user-list", &context)
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

#[put("/<id>")]
pub fn update_user_rt(id: String) -> String {
    format!("Update info for user {}", id)
}

#[delete("/<id>")]
pub fn delete_user_rt(id: String) -> String {
    format!("Delete user {}", id)
}
