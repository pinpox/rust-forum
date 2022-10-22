use rocket::form::Form;
// use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket::*;
// use rocket_dyn_templates::Template;

use crate::models::post::*;

#[get("/")]
pub fn post_list_rt() -> String {
    "List of posts".to_string()
}

#[post("/new", data = "<data>")]
pub fn create_post_rt(data: rocket::form::Result<Form<NewPost>>) -> Flash<Redirect> {
    let new_post = match data {
        Err(errors) => {
    println!("err 1");
            let errs: Vec<String> = errors
                .iter()
                .map(|e| format!("{}", e.name.as_ref().expect(", ")))
                .collect();
            return Flash::error(
                Redirect::to(uri!("/error")),
                format!("Error creating post: {}", errs.join(", ")),
            );
        }
        Ok(d) => NewPost {
            user_id: d.user_id,
            content: d.content.to_string(),
            created_at: 0, //TODO
            topic_id: d.topic_id,
        },
    };

    println!("CREATING");

    match create_post(new_post) {
        Ok(..) => Flash::success(
            Redirect::to(uri!( crate::routes::topic::info_topic_rt(1))),
            format!("Post created!")),
        Err(e) => Flash::error(
            Redirect::to(uri!("/error")),
            format!("Error creating post: {}", e),
        ),
    }
}

#[post("/<id>")]
pub fn update_post_rt(id: String) -> String {
    format!("Update info for post {}", id)
}

#[delete("/<id>")]
pub fn delete_post_rt(id: String) -> String {
    format!("Delete post {}", id)
}
