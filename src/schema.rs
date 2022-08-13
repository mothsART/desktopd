table! {
    app (id) {
        id -> Integer,
        title -> Text,
        path -> Text,
        generic_title -> Nullable<Text>,
        comment -> Nullable<Text>,
        exec -> Nullable<Text>,
        try_exec -> Nullable<Text>,
        icon_path -> Nullable<Text>,
    }
}

table! {
    comments (id) {
        id -> Integer,
        app_id -> Integer,
        title -> Text,
        lang -> Text,
    }
}

table! {
    keywords (id) {
        id -> Integer,
        app_id -> Integer,
        key -> Text,
        lang -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(app, comments, keywords,);
