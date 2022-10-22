use rocket_dyn_templates::Template;
use rocket::request::FlashMessage;
extern crate serde_json;
use serde_json::json;
// use rocket::Request;

#[get("/error")]
pub fn error_rt( flash: Option<FlashMessage>) -> Template {
    Template::render("error", json!({"flash": flash}))
}

// #[catch(422)]
// pub fn not_parsable(req: &Request) {
//     println!("{:#?}", req);
// }

#[catch(404)]
pub fn not_found() -> Template {
    Template::render("error", json!({
        "header": "Not found",
    }))
}


