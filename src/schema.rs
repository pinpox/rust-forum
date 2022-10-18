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
        subject -> Nullable<Text>,
        content -> Text,
        topic_id -> Integer,
        created_at -> Integer,
        updated_at -> Nullable<Integer>,
    }
}

diesel::table! {
    topics (id) {
        id -> Integer,
        board_id -> Integer,
        subject -> Text,
        is_sticky -> Bool,
        is_locked -> Bool,
        created_at -> Timestamp,
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
