// @generated automatically by Diesel CLI.

diesel::table! {
    boards (id) {
        id -> Integer,
        forum_id -> Integer,
        name -> Text,
        description -> Text,
        updated_at -> Integer,
        position -> Integer,
        is_locked -> Bool,
    }
}

diesel::table! {
    forums (id) {
        id -> Integer,
        name -> Text,
        position -> Integer,
        is_locked -> Bool,
    }
}

diesel::table! {
    posts (id) {
        id -> Integer,
        user_id -> Integer,
        content -> Text,
        topic_id -> Integer,
        created_at -> Integer,
    }
}

diesel::table! {
    topics (id) {
        id -> Integer,
        board_id -> Integer,
        user_id -> Integer,
        subject -> Text,
        content -> Text,
        is_sticky -> Bool,
        is_locked -> Bool,
        created_at -> Integer,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
        about -> Text,
        picture -> Text,
        password -> Text,
        is_admin -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    boards,
    forums,
    posts,
    topics,
    users,
);
