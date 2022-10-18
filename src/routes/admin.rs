use crate::models::board::*;
use crate::models::forum::*;

use rocket_dyn_templates::Template;
extern crate serde_json;

use rocket::form::Form;

use serde_json::json;

#[get("/")]
pub fn manage_rt() -> Template {
    Template::render("admin", json!({}))
}
