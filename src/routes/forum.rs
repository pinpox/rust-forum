use crate::models::board::*;
use crate::models::forum::*;

use rocket_dyn_templates::Template;
extern crate serde_json;

use rocket::form::Form;

use serde_json::json;

//TODO show error on invalid inputs
#[post("/", data = "<data>")]
pub fn create_forum_rt(data: rocket::form::Result<Form<NewForum>>) -> Template {
    let new_forum = match data {
        Err(errors) => {
            let errs: Vec<String> = errors
                .iter()
                .map(|e| format!("{}", e.name.as_ref().expect(", ")))
                .collect();
            return Template::render(
                "forum-new",
                json!({ "message": format!("Invalid values entered for: {}", errs.join(", ")) }),
            );
        }
        Ok(d) => NewForum {
            name: d.name.to_string(),
            position: d.position,
            is_locked: d.is_locked,
        },
    };

    Template::render(
        "forum-new",
        match create_forum(new_forum) {
            Ok(_n) => json!({ "message": format!("Forum created succesfully!") }),
            Err(e) => json!({ "message": format!("Error creating forum: {}", e) }),
        },
    )
}

#[get("/new")]
pub fn new_forum_rt() -> Template {
    Template::render("forum-new", json!({}))
}

#[get("/<id>")]
pub fn info_forum_rt(id: i32) -> Template {
    Template::render(
        "forum",
        match get_forum_by_id(id) {
            Err(e) => json!({"message": e.to_string()}),
            Ok(f) => match get_forum_boards(f.id) {
                Err(e) => json!({"message": e.to_string()}),
                Ok(b) => {
                    json!({
                        "boards": b,
                        "forums": vec![f],
                    })
                }
            },
        },
    )
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

#[get("/")]
pub fn forum_list_rt() -> Template {

    Template::render(
        "forums",
        match get_forums() {
            Err(e) => json!({"message": e.to_string()}),
            Ok(f) => match get_boards() {
                Err(e) => json!({"message": e.to_string()}),
                Ok(b) => json!({
                    "boards": b,
                    "forums": f
                }),
            },
        },
    )
}
