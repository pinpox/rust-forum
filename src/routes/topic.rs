use crate::models::board::*;
use crate::models::post::*;
use crate::models::topic::*;
use crate::models::user::*;
use crate::routes::other::*;

// use rocket::*;
use rocket::form::Form;
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket_dyn_templates::Template;
use serde_json::json;

// #[get("/")]
// pub fn topic_list_rt() -> String {
//     "List of topics".to_string()
// }

#[post("/", data = "<data>")]
pub fn create_topic_rt(
    data: rocket::form::Result<Form<NewTopicRequest>>,
    user: User,
) -> Flash<Redirect> {
    let new_topic = match data {
        Err(errors) => {
            let errs: Vec<String> = errors
                .iter()
                .map(|e| format!("{}", e.name.as_ref().expect(", ")))
                .collect();

            return Flash::error(
                Redirect::to(uri!(new_topic_rt)),
                format!(
                    "Failed to create topic. Invalid values for: {}",
                    errs.join(", ")
                ),
            );
        }
        Ok(d) => NewTopic {
            created_at: 0, // TODO
            board_id: d.board_id,
            user_id: user.id,
            is_locked: d.is_locked,
            is_sticky: d.is_sticky,
            subject: d.subject.to_string(),
            content: d.content.to_string(),
        },
    };

    match create_topic(new_topic) {
        Ok(_n) => Flash::success(Redirect::to(uri!(new_topic_rt)), format!("Topic created!")),

        Err(e) => Flash::error(
            Redirect::to(uri!(new_topic_rt)),
            format!("Failed to create topic: {}", e),
        ),
    }
}

#[get("/new")]
pub fn new_topic_rt(flash: Option<FlashMessage>) -> Result<Template, Flash<Redirect>> {
    match get_boards() {
        Err(e) => Err(Flash::error(
            Redirect::to(uri!(error_rt)),
            format!("Failed to load boards: {}", e),
        )),
        Ok(b) => Ok(Template::render(
            "topic-new",
            json!({ "boards": b, "flash": flash, }),
        )),
    }
}

// pub fn new_topic_rt() -> Template {
//     render_new("".to_string())
// }

#[get("/<id>")]
pub fn info_topic_rt(id: i32) -> Result<Template, Flash<Redirect>> {
    match get_topic_by_id(id) {
        Err(e) => Err(Flash::error(
            Redirect::to(uri!(error_rt)),
            format!("Failed to get topic with ID: {} ({:#?})", id, e),
        )),
        Ok(t) => match get_topic_posts(t.id) {
            Err(e) => Err(Flash::error(
                Redirect::to(uri!(error_rt)),
                format!("Failed to get posts for topic with ID: {} ({:#?})", id, e),
            )),
            Ok(p) => match get_topic_users(t.id) {
                Err(e) => Err(Flash::error(
                    Redirect::to(uri!(error_rt)),
                    format!("Failed to get users for topic with ID: {} ({:#?})", id, e),
                )),

                Ok(u) => Ok(Template::render(
                    "topic",
                    json!({
                        "posts": p,
                        "topic": t,
                        "users": u
                    }),
                )),
            },
        },
    }
}

// #[put("/<id>")]
// pub fn update_topic_rt(id: String) -> String {
//     format!("Update info for topic {}", id)
// }

// #[delete("/<id>")]
// pub fn delete_topic_rt(id: String, user: User) -> String {
//     format!("Delete topic {}", id)
// }
