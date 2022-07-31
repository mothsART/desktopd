table! {
    app (id) {
        id -> Nullable<Integer>,
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
        id -> Nullable<Integer>,
        app_id -> Nullable<Integer>,
        title -> Text,
        lang -> Text,
    }
}

table! {
    generic_names (id) {
        id -> Nullable<Integer>,
        app_id -> Nullable<Integer>,
        title -> Text,
        lang -> Text,
    }
}

table! {
    keywords (id) {
        id -> Nullable<Integer>,
        app_id -> Nullable<Integer>,
        key -> Text,
        lang -> Nullable<Text>,
    }
}

table! {
    names (id) {
        id -> Nullable<Integer>,
        app_id -> Nullable<Integer>,
        title -> Text,
        lang -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    app,
    comments,
    generic_names,
    keywords,
    names,
);
