// use rocket::*;

use rocket::{get, post};

// use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};

extern crate serde_json;
use rocket_dyn_templates::Template;
use serde_json::json;

use crate::models::user::*;
use rocket::form::Form;
// use std::collections::HashMap;

// #[get("/new")]
// pub fn new_user_rt(flash: Option<FlashMessage>) -> Template {
//     Template::render("forum-new", json!({ "flash": flash }))
// }

//TODO show error on invalid inputs
#[post("/complete", data = "<data>")]
pub fn complete_user_rt(
    subject: UserSubject,
    data: rocket::form::Result<Form<UserUpdateData>>,
) -> Flash<Redirect> {
    let new_user = match data {
        Err(errors) => {
            let errs: Vec<String> = errors
                .iter()
                .map(|e| format!("{}", e.name.as_ref().expect(", ")))
                .collect();
            panic!();
            // return Flash::error(
            //     Redirect::to(uri!("/TODO/new")),
            //     format!("Error creating forum: {}", errs.join(", ")),
            // );
        }

        Ok(d) => {
            User {
                id: subject.subject,
                name: d.name.to_owned(),
                about: d.about.to_owned(),
                picture: d.picture.to_owned(),
                is_admin: false, // TODO FIX THIS: will remove admin rights on user update
            }
        }
    };

    match create_or_update_user(new_user) {
        Ok(_n) => Flash::success(Redirect::to(uri!("/forums")), "User updated succcessfully!"),
        Err(e) => Flash::error(
            Redirect::to(uri!("/TODO")),
            format!("Error creating user: {}", e),
        ),
    }
}

#[get("/account")]
pub fn edit_user_rt(user: User) -> Template {
    Template::render("user-edit", json!({ "user": user }))
}

#[get("/<id>")]
pub fn info_user_rt(id: String) -> Template {
    Template::render(
        "user",
        match get_by_id(&id) {
            Err(e) => json!({"message": e.to_string()}),
            // Ok(u) => match get_user_topics(id) {
            // Err(e) => json!({"message": e.to_string()}),
            // Ok(f) => match get_users() {
            // Err(e) => json!({"message": e.to_string()}),
            Ok(u) => json!({
                // "posts": p,
                "user": u
            }),
            // },
        },
        // }
    )
}

#[post("/update/<id>")]
pub fn update_user_rt(id: String) -> String {
    format!("Update info for user {}", id)
}

#[get("/delete/<id>")]
pub fn delete_user_rt(id: String) -> String {
    format!("Delete user {}", id)
}
