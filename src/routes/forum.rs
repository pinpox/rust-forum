use rocket::form::Form;
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket_dyn_templates::Template;
extern crate serde_json;
use serde_json::json;

use crate::models::board::*;
use crate::models::forum::*;

//TODO show error on invalid inputs
#[post("/", data = "<data>")]
pub fn create_forum_rt(data: rocket::form::Result<Form<NewForum>>) -> Flash<Redirect> {
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

#[get("/new")]
pub fn new_forum_rt(flash: Option<FlashMessage>) -> Template {
    Template::render("forum-new", json!({ "flash": flash }))
}

#[get("/<id>")]
pub fn info_forum_rt(id: i32) -> Result<Template, Flash<Redirect>> {
    match get_forum_by_id(id) {
        Err(e) => Err(Flash::error(
            Redirect::to(uri!("/error")),
            format!("Error creating forum: {}", e),
        )),

        Ok(f) => match get_forum_boards(f.id) {
            Err(e) => Err(Flash::error(
                Redirect::to(uri!("/error")),
                format!("Error creating forum: {}", e),
            )),

            Ok(b) => Ok(Template::render(
                "forum",
                json!({ "boards": b, "forum": f }),
            )),
        },
    }
}

#[put("/<id>")]
pub fn update_forum_rt(id: String) -> String {
    //TODO
    format!("Update info for forum {}", id)
}

#[delete("/<id>")]
pub fn delete_forum_rt(id: String) -> String {
    //TODO
    format!("Delete forum {}", id)
}

use crate::User;

#[get("/")]
pub fn forum_list_rt(user: Option<User>) -> Template {

    let user_name: Option<String> = match user {
        Some(u) => Some(u.name.to_string()),
        None => None
    };


    Template::render(
        "forums",
        match get_forums() {
            Err(e) => json!({"message": e.to_string()}),
            Ok(f) => match get_boards() {
                Err(e) => json!({"message": e.to_string()}),
                Ok(b) => json!({
                    "boards": b,
                    "forums": f,
                    "user": user_name
                }),
            },
        },
    )
}
