use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::db::establish_connection;
use crate::models::forum::Forum;
use crate::schema::boards;


#[derive(Debug, Queryable, Identifiable, AsChangeset, Serialize, Deserialize, Associations)]
#[diesel(belongs_to(Forum))]
pub struct Board {
    pub id: i32,
    pub forum_id: i32,
    pub name: String,
    pub description: String,
    pub updated_at: i32,
    pub position: i32,
    pub is_locked: bool,
}

#[derive(Debug, Insertable, FromForm)]
#[diesel(table_name = boards)]
pub struct NewBoard {
    pub forum_id: i32,
    pub name: String,
    pub description: String,
    pub updated_at: i32,
    pub position: i32,
    pub is_locked: bool,
}

pub fn get_forum_boards(f_id: i32) -> Result<Vec<Board>, diesel::result::Error> {
    use crate::schema::boards::dsl::*;
    let mut connection = establish_connection();
    boards
        .filter(forum_id.eq(f_id))
        .load::<Board>(&mut connection)
}

pub fn create_board(board: NewBoard) -> Result<usize, diesel::result::Error> {
    println!("Creating board: {:?}", board);
    use crate::schema::boards::dsl::*;
    let mut connection = establish_connection();


    let new_board = NewBoard {
        forum_id: board.forum_id,
        name: board.name,
        description: board.description,
        updated_at: 0,// TODO
        is_locked: board.is_locked,
        position: board.position,
    };

    println!("Creating board: {:?}", new_board);

    let ins = diesel::insert_into(boards)
        .values(&new_board)
        .execute(&mut connection);

    println!("{:#?}", ins);

    ins
}

pub fn update_board(board: Board) {
    println!("Updating board: {:?}", board);
    use crate::schema::boards::dsl::*;
    let mut connection = establish_connection();

    diesel::update(boards.find(board.id))
        .set(&board)
        .execute(&mut connection)
        .expect("Error updating board");
}

// TODO Should we really return the raw diesel::result::Error here?
// Maybe wrap it in something?
pub fn get_boards() -> Result<Vec<Board>, diesel::result::Error> {
    use crate::schema::boards::dsl::*;
    let mut connection = establish_connection();
    let b = boards
        // .filter(removed.eq(false))
        .load::<Board>(&mut connection);
    println!("Found boards: {:#?}", b);

    b
    // .expect("Error loading boards")
}
