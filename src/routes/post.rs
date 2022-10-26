use crate::models::post::*;
use crate::models::user::User;
use crate::routes::other::*;
use crate::routes::topic::*;

use rocket::form::Form;
use rocket::response::{Flash, Redirect};
// use rocket::*;
use rocket_dyn_templates::Template;

use serde_json::json;

#[get("/")]
pub fn post_list_rt(user: Option<User>) -> Template {
    Template::render(
        "posts",
        match get_posts() {
            // Err(e) => json!({"message": e.to_string()}),
            // Ok(f) => match get_users() {
            Err(e) => json!({"message": e.to_string()}),
            Ok(p) => json!({
                "posts": p,
                "user": user
            }),
            // },
        },
    )
}

#[post("/new", data = "<data>")]
pub fn create_post_rt(
    data: rocket::form::Result<Form<NewPostRequest>>,
    user: User,
) -> Flash<Redirect> {
    let new_post = match data {
        Err(errors) => {
            let errs: Vec<String> = errors
                .iter()
                .map(|e| format!("{}", e.name.as_ref().expect(", ")))
                .collect();
            return Flash::error(
                Redirect::to(uri!(error_rt)),
                format!("Error creating post: {}", errs.join(", ")),
            );
        }
        Ok(d) => NewPost {
            user_id: user.id,
            content: d.content.to_string(),
            created_at: 0, //TODO
            topic_id: d.topic_id,
        },
    };

    let post_topic = new_post.topic_id;

    match create_post(new_post) {
        Ok(..) => Flash::success(
            Redirect::to(uri!("/topics", info_topic_rt(post_topic))),
            format!("Post created!"),
        ),
        Err(e) => Flash::error(
            Redirect::to(uri!(error_rt)),
            format!("Error creating post: {}", e),
        ),
    }
}

// #[post("/<id>")]
// pub fn update_post_rt(id: String) -> String {
//     format!("Update info for post {}", id)
// }

// #[delete("/<id>")]
// pub fn delete_post_rt(id: String) -> String {
//     format!("Delete post {}", id)
// }
