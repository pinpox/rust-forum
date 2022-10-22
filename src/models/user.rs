use diesel::prelude::*;

// use serde::{Deserialize, Serialize};

// use crate::db::establish_connection;
// use crate::schema::forums;

use crate::hatch;
use hatch::OidcHatch;
use rocket::{
    info_,
    request::{FromRequest, Outcome},
    Request,
};
use rocket_airlock::{Airlock, Hatch};

// use crate::models::user::User;

use serde::{Deserialize, Serialize};

use crate::db::establish_connection;
use crate::schema::users;

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct UserSubject {
    pub subject: String,
}

#[derive(Debug, Queryable, Identifiable, AsChangeset, Serialize, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: String,
    pub name: String,
    pub about: Option<String>,
    pub picture: Option<String>,
    // #[field(default = false)]
    pub is_admin: bool,
}

#[derive(Debug, FromForm)]
pub struct UserUpdateData {
    pub name: String,
    pub about: Option<String>,
    pub picture: Option<String>,
}

// #[derive(Debug, Insertable, FromForm)]
// #[diesel(table_name = forums)]
// pub struct NewUser {

//     pub id: String,
//     pub name: String,
//     pub about: String,
//     pub picture: String,
//     pub is_admin: bool,

//     #[field(validate = len(2..))]
//     pub name: String,
//     #[field(default = 0)]
//     pub position: i32,
//     pub is_locked: bool,
// }

pub fn get_by_id(u_id: &String) -> Result<User, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    let mut connection = establish_connection();
    users.find(u_id).first(&mut connection)
}

// Selects all users that participate in a topic (creator + repliers)
pub fn get_topic_users(t_id: i32) -> Result<Vec<User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    let mut connection = establish_connection();

    //TODO!!!
    //Don't return all users, just the ones who's ID is one of:
    // - topic.user_id
    // - SELECT user_id FROM posts WHERE topic_id = topic.user_id

    // let distinct_user_ids = users.select(user_id).distinct()
    // .filter(removed.eq(false))
    // .load::<String>(&connection)?;

    users.load::<User>(&mut connection)
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let cookies = request.cookies();
        match cookies.get_private("oicd_access_token") {
            Some(token_cookie) => {
                let hatch = request
                    .guard::<Airlock<OidcHatch>>()
                    .await
                    .expect(&format!(
                        "Hatch '{}' was not installed into the airlock.",
                        OidcHatch::name()
                    ))
                    .hatch;

                if hatch.validate_access_token(token_cookie.value()) {
                    let subject = cookies.get_private("sub").unwrap().value().to_string();

                    info_!("Knonw User '{}' logged in!", &subject);
                    match crate::models::user::get_by_id(&subject) {
                        Ok(u) => return Outcome::Success(u),
                        Err(_e) => return Outcome::Forward(()), // panic!("User '{}' not found in DB: {}", subject.to_string(), e)
                    }
                }

                Outcome::Forward(())
            }
            _ => Outcome::Forward(()),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserSubject {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let cookies = request.cookies();
        match cookies.get_private("oicd_access_token") {
            Some(token_cookie) => {
                let hatch = request
                    .guard::<Airlock<OidcHatch>>()
                    .await
                    .expect(&format!(
                        "Hatch '{}' was not installed into the airlock.",
                        OidcHatch::name()
                    ))
                    .hatch;

                if hatch.validate_access_token(token_cookie.value()) {
                    let s = cookies.get_private("sub").unwrap().value().to_string();

                    info_!("New User '{}' logged in!", &s);
                    return Outcome::Success(UserSubject { subject: s });
                }

                Outcome::Forward(())
            }
            _ => Outcome::Forward(()),
        }
    }
}

pub fn create_or_update_user(user: User) -> Result<usize, diesel::result::Error> {
    println!("Creating user: {:?}", user);
    use crate::schema::users::dsl::*;
    let mut connection = establish_connection();

    diesel::insert_into(users)
        .values(&user)
        .on_conflict(id)
        .do_update()
        .set(&user)
        .execute(&mut connection)
}

// pub fn update_user(forum: Forum) {
//     println!("Updating forum: {:?}", forum);
//     use crate::schema::forums::dsl::*;
//     let mut connection = establish_connection();

//     diesel::update(forums.find(forum.id))
//         .set(&forum)
//         .execute(&mut connection)
//         .expect("Error updating Forum");
// }
