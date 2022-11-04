#![feature(proc_macro_hygiene, decl_macro)]
#![allow(unused_attributes)]

#[macro_use]
extern crate rocket;

#[macro_use]
use rocket::*;
// use rocket_contrib::helmet::SpaceHelmet;
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
extern crate serde_json;

use rocket::request::FlashMessage;
// use rocket::response::{Flash};

pub mod db;
pub mod models;
pub mod schema;

mod routes;

use rocket::response::Flash;
use rocket::{get, info_, response::Redirect, routes};
use rocket_airlock::Airlock;
// use user::User;

use crate::models::user::{User, UserSubject};
use serde_json::json;


use rocket_dyn_templates::tera::{try_get_value, Result};
use serde_json::value::{to_value, Value};
use std::collections::HashMap;

// For code syntax highlighting
// use comrak::{markdown_to_html_with_plugins, plugins::syntect::SyntectAdapter, ComrakPlugins};
use comrak::{markdown_to_html, ComrakOptions};

mod hatch;

#[get("/login")]
pub fn index_auth(user: User) -> Flash<Redirect> {
    info_!("Successful login from: {}", user.name);
    Flash::success(
        Redirect::to(uri!("/forums")),
        format!("Welcome, {}", user.name),
    )
}

#[get("/login", rank = 2)]
pub fn index_auth_not_complete(subject: UserSubject, flash: Option<FlashMessage>) -> Template {
    info_!("Incomplete user loggend in: {}", subject.subject);
    Template::render(
        "user-edit",
        json!({
            "subject": subject.subject,
            "flash": flash,
        }),
    )
}

#[get("/login", rank = 3)]
pub fn index_auth_anon() -> Redirect {
    info_!("Anonymous user requested / -> redirecting to /authenticate");
    Redirect::to("/authenticate")
}

pub fn markdown_filter(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
    let s = try_get_value!("markdown_filter", "value", String, value);

    // To enable code syntax highlighting (slow)
    // let adapter = SyntectAdapter::new("base16-ocean.dark");
    // let mut plugins = ComrakPlugins::default();
    // plugins.render.codefence_syntax_highlighter = Some(&adapter);
    // let outstring = markdown_to_html_with_plugins(&s, &ComrakOptions::default(), &plugins);

    let outstring = markdown_to_html(&s, &ComrakOptions::default());

    Ok(to_value(&outstring).unwrap())
}

// pub fn markdown_filter<E>(value: Value, _: HashMap<String, Value>) -> Result<Value, E> {
//     let s = "";
//     Ok(to_value(1))
// }

// &'a serde_json::Value, &'b HashMap<std::string::String, serde_json::Value>
// pub fn markdown_filter(value: User, map: HashMap<String, serde_json::Value>) -> String {
//     "test".to_string()

//     // let s = try_get_value!("upper", "value", String, value);
//     // Ok(to_value(markdown::to_html(s.as_str())))
// }

#[launch]
pub fn rocket_builder() -> rocket::Rocket<Build> {
    rocket::build()
        .register("/", catchers![routes::other::not_found])
        .attach(Template::custom(|engines| {
            engines.tera.register_filter("markdown", markdown_filter)
        }))
        .mount(
            "/",
            routes![
                index_auth,
                index_auth_not_complete,
                index_auth_anon,
                routes::forum::forum_list_rt,
            ],
        )
        .attach(Airlock::<hatch::OidcHatch>::fairing())
        .mount("/ping", routes![routes::ping::ping_fn])
        // .mount("/", routes![routes::forum::forum_list_rt])
        .mount(
            "/users",
            routes![
                // routes::user::user_list_rt,
                // routes::user::new_user_rt,
                routes::user::complete_user_rt,
                routes::user::info_user_rt,
                // routes::user::update_user_rt,
                // routes::user::delete_user_rt
            ],
        )
        .mount(
            "/forums",
            routes![
                routes::forum::forum_list_rt,
                routes::forum::new_forum_rt,
                routes::forum::create_forum_rt,
                routes::forum::info_forum_rt,
                routes::forum::update_forum_rt,
                routes::forum::delete_forum_rt
            ],
        )
        .mount(
            "/boards",
            routes![
                // routes::board::board_list_rt,
                routes::board::new_board_rt,
                routes::board::create_board_rt,
                routes::board::info_board_rt,
                // routes::board::update_board_rt,
                // routes::board::delete_board_rt
            ],
        )
        .mount(
            "/topics",
            routes![
                routes::topic::info_topic_rt,
                // routes::topic::topic_list_rt,
                routes::topic::new_topic_rt,
                routes::topic::create_topic_rt,
                // routes::topic::update_topic_rt,
                // routes::topic::delete_topic_rt
            ],
        )
        .mount(
            "/posts",
            routes![
                routes::post::post_list_rt,
                routes::post::create_post_rt,
                // routes::post::info_post_rt,
                // routes::post::update_post_rt,
                // routes::post::delete_post_rt
            ],
        )
        .mount("/admin", routes![routes::admin::manage_rt])
        .mount(
            "/",
            routes![
                routes::other::error_rt,    // Error page (/error)
                routes::user::edit_user_rt  // Edit own account (/account)
            ],
        )
        .mount("/static", FileServer::from("static/"))
}
