use rocket_dyn_templates::Template;
extern crate serde_json;
use crate::models::user::AdminUser;
use serde_json::json;

#[get("/")]
pub fn manage_rt(user: AdminUser) -> Template {
    Template::render("admin", json!({ "user": user }))
}
