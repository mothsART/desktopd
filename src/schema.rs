table! {
    app (id) {
        id -> Int4,
        title -> Text,
        path -> Text,
        generic_title -> Nullable<Text>,
        exec -> Text,
        try_exec -> Nullable<Text>,
        icon_path -> Nullable<Text>,
    }
}

table! {
    comments (id) {
        id -> Int4,
        app_id -> Int4,
        title -> Nullable<Text>,
        lang -> Text,
    }
}

table! {
    keywords (id) {
        id -> Int4,
        app_id -> Int4,
        key -> Text,
        lang -> Nullable<Text>,
    }
}

joinable!(comments -> app (app_id));
joinable!(keywords -> app (app_id));

allow_tables_to_appear_in_same_query!(app, comments, keywords,);
