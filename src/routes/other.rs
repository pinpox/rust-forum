use rocket_dyn_templates::Template;
extern crate serde_json;
use serde_json::json;

#[get("/error")]
pub fn error_rt() -> Template {
    Template::render("error", json!({}))
}
