use rocket::*;

use crate::models::board::*;
use crate::models::post::*;
use crate::models::topic::*;
use crate::models::user::*;

use rocket_dyn_templates::Template;
use serde_json::json;

use rocket::form::Form;

#[get("/")]
pub fn topic_list_rt() -> String {
    "List of topics".to_string()
}

#[post("/", data = "<data>")]
pub fn create_topic_rt(data: rocket::form::Result<Form<NewTopic>>) -> Template {
    let new_topic = match data {
        Err(errors) => {
            let errs: Vec<String> = errors
                .iter()
                .map(|e| format!("{}", e.name.as_ref().expect(", ")))
                .collect();
            return Template::render(
                "topic-new",
                json!({ "message": format!("Invalid values entered for: {}", errs.join(", ")) }),
            );
        }
        Ok(d) => NewTopic {
            created_at: 0, // TODO
            board_id: d.board_id,
            user_id: 0, //TODO
            is_locked: d.is_locked,
            is_sticky: d.is_sticky,
            subject: d.subject.to_string(),
            content: d.content.to_string(),
        },
    };

    match create_topic(new_topic) {
        Ok(_n) => render_new(format!("Topic created")),
        Err(e) => render_new(format!("Failed to create topic: {}", e)),
    }
}

fn render_new(message: String) -> Template {
    Template::render(
        "topic-new",
        match get_boards() {
            Err(e) => json!({"message": e.to_string()}),
            Ok(b) => json!({
                "boards": b,
                "message": message,
            }),
        },
    )
}

#[get("/new")]
pub fn new_topic_rt() -> Template {
    render_new("".to_string())
}

#[get("/<id>")]
pub fn info_topic_rt(id: i32) -> Template {
    Template::render(
        "topic",
        match get_topic_by_id(id) {
            Err(e) => {
                println!("Failed to get topic ID: {} ({:#?})", id, e);
                json!({"message": e.to_string()})
            }
            Ok(t) => match get_topic_posts(t.id) {
                Err(e) => {
                    println!("{:#?}", e);
                    json!({"message": e.to_string()})
                }
                Ok(p) => match get_topic_users(t.id) {
                    Err(e) => {
                        println!("{:#?}", e);
                        json!({"message": e.to_string()})
                    }
                    Ok(u) => {
                        println!("{:#?}", t);
                        json!({
                            "posts": p,
                            "topic": t,
                            "users": u,
                        })
                    }
                },
            },
        },
    )
}

#[put("/<id>")]
pub fn update_topic_rt(id: String) -> String {
    format!("Update info for topic {}", id)
}

#[delete("/<id>")]
pub fn delete_topic_rt(id: String) -> String {
    format!("Delete topic {}", id)
}
