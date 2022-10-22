// use rocket::*;

use rocket::{get, post};


use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};

extern crate serde_json;
// use serde_json::json;
use rocket_dyn_templates::Template;

use crate::models::user::*;
use std::collections::HashMap;


// #[get("/new")]
// pub fn new_user_rt(flash: Option<FlashMessage>) -> Template {
//     Template::render("forum-new", json!({ "flash": flash }))
// }

//TODO show error on invalid inputs
#[post("/complete", data = "<data>")]
pub fn complete_user_rt(data: rocket::form::Result<Form<NewForum>>) -> Flash<Redirect> {
    let new_forum = match data {
        Err(errors) => {
            let errs: Vec<String> = errors
                .iter()
                .map(|e| format!("{}", e.name.as_ref().expect(", ")))
                .collect();
            return Flash::error(
                Redirect::to(uri!("/forums/new")),
                format!("Error creating forum: {}", errs.join(", ")),
            );
        }
        Ok(d) => NewForum {
            name: d.name.to_string(),
            position: d.position,
            is_locked: d.is_locked,
        },
    };

    match create_forum(new_forum) {
        Ok(_n) => Flash::success(
            Redirect::to(uri!("/forums/new")),
            "Forum created succcessfully!",
        ),
        Err(e) => Flash::error(
            Redirect::to(uri!("/forums/new")),
            format!("Error creating forum: {}", e),
        ),
    }
}

#[get("/")]
pub fn user_list_rt() -> Template {

    //TODO remove?
    let user2 = User {
        id: "1".to_string(),
        about: "About user 2".to_string(),
        name: "Username2".to_string(),
        picture: "pic2".to_string(),
        is_admin: false,
    };

    let user1 = User {
        id: "2".to_string(),
        about: "About user 1".to_string(),
        name: "Username1".to_string(),
        picture: "pic1".to_string(),
        is_admin:true,
    };

    let users = [user1, user2];
    let mut context = HashMap::new();
    context.insert("users", &users);

    Template::render("user-list", &context)
}

// #[get("/users")]
// pub fn user_list_rt() -> String {
//     "List of users".to_string()
// }

// #[post("/")]
// pub fn new_user_rt() -> String {
//     "Creation of new user".to_string()
// }

#[get("/<id>")]
pub fn info_user_rt(id: String) -> String {
    format!("Info for user {}", id)
}

#[post("/update/<id>")]
pub fn update_user_rt(id: String) -> String {
    format!("Update info for user {}", id)
}

#[get("/delete/<id>")]
pub fn delete_user_rt(id: String) -> String {
    format!("Delete user {}", id)
}


