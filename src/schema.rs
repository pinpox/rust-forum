// @generated automatically by Diesel CLI.

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
    forums,
    posts,
    topics,
    users,
);