use rocket_dyn_templates::Template;
extern crate serde_json;
use crate::models::user::AdminUser;
use serde_json::json;

#[get("/")]
pub fn manage_rt(user: AdminUser) -> Template {
    // if user.is_admin {
        // println!("admin pane access by: {:#?}", user);
        Template::render("admin", json!({ "user": user }))
    // } else {
    //     Template::render(
    //         "error",
    //         json!({ "flash": { "kind": "error", "message": "You need to be an admin to do this." } }),
    //     )
    // }
}
