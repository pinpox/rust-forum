use rocket_dyn_templates::Template;
extern crate serde_json;
use serde_json::json;

#[get("/")]
pub fn manage_rt() -> Template {
    Template::render("admin", json!({}))
}
