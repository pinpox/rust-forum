#![feature(proc_macro_hygiene, decl_macro)]
#![allow(unused_attributes)]

#[macro_use]
extern crate rocket;

#[macro_use]
use rocket::*;
// use rocket_contrib::helmet::SpaceHelmet;
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;

pub mod db;
pub mod models;
pub mod schema;

mod routes;

// #[catch(422)]
// fn not_parsable(req: &Request) {
//     println!("{:#?}", req);
// }

#[launch]
pub fn rocket_builder() -> rocket::Rocket<Build> {
    rocket::build()
        // .register(catchers![not_parsable])
        .attach(Template::fairing())
        // .attach(SpaceHelmet::default())
        .mount("/ping", routes![routes::ping::ping_fn])
        .mount("/", routes![routes::forum::forum_list_rt])
        .mount(
            "/users",
            routes![
                routes::user::user_list_rt,
                routes::user::new_user_rt,
                // routes::user::create_user_rt,
                routes::user::info_user_rt,
                routes::user::update_user_rt,
                routes::user::delete_user_rt
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
                routes::board::update_board_rt,
                routes::board::delete_board_rt
            ],
        )
        .mount(
            "/topics",
            routes![
                routes::topic::topic_list_rt,
                routes::topic::new_topic_rt,
                routes::topic::create_topic_rt,
                routes::topic::info_topic_rt,
                routes::topic::update_topic_rt,
                routes::topic::delete_topic_rt
            ],
        )
        .mount(
            "/posts",
            routes![
                routes::post::post_list_rt,
                routes::post::new_post_rt,
                routes::post::info_post_rt,
                routes::post::update_post_rt,
                routes::post::delete_post_rt
            ],
        )
        .mount("/admin", routes![routes::admin::manage_rt])
        .mount("/", routes![routes::other::error_rt])
        .mount("/static", FileServer::from("static/"))
}
