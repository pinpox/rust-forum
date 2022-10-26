use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::Integer;

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

#[derive(
    Debug, Queryable, Identifiable, AsChangeset, Serialize, Deserialize, Insertable, QueryableByName,
)]
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

#[derive(Debug, FromForm, Serialize)]
pub struct AdminUser {
    pub id: String,
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
    let mut connection = establish_connection();

    // TODO use diesel query DSL instead of raw sql_query
    //This is currently not possible due to: https://github.com/diesel-rs/diesel/issues/3380
    sql_query("select * from users where id in (select user_id from topics where id = ? union select user_id from posts where topic_id = ?)")
     .bind::<Integer, _>(t_id)
     .bind::<Integer, _>(t_id)
     .load::<User>(&mut connection)
}

pub async fn user_from_request(request: &Request<'_>) -> Option<User> {
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

                info_!("Known User '{}' logged in!", &subject);
                return match crate::models::user::get_by_id(&subject) {
                    Ok(u) => {
                        info_!("found user with name: {}", u.name);
                        Some(u)
                    },
                    Err(..) => None,
                };
            }
            None
            // Outcome::Forward(())
        }
        _ => None,
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match user_from_request(request).await {
            Some(u) => Outcome::Success(u),
            None => Outcome::Forward(()),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match user_from_request(request).await {
            Some(u) => {
                if u.is_admin {
                    return Outcome::Success(AdminUser {
                        id: u.id,
                        name: u.name,
                        about: u.about,
                        picture: u.picture,
                    });
                }
                Outcome::Forward(())
            }
            None => Outcome::Forward(()),
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
