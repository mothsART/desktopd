table! {
    app (id) {
        id -> Integer,
        title -> Text,
        path -> Text,
        generic_title -> Nullable<Text>,
        exec -> Nullable<Text>,
        try_exec -> Nullable<Text>,
        icon_path -> Nullable<Text>,
    }
}

table! {
    app_locale (id) {
        id -> Integer,
        app_id -> Integer,
        locale_id -> Integer,
    }
}

table! {
    comments (id) {
        id -> Integer,
        app_id -> Integer,
        locale_id -> Integer,
        title -> Text,
    }
}

table! {
    keywords (id) {
        id -> Integer,
        app_id -> Integer,
        locale_id -> Integer,
        key -> Text,
    }
}

table! {
    locale (id) {
        id -> Integer,
        key -> Text,
    }
}

joinable!(app_locale -> app (app_id));
joinable!(app_locale -> locale (locale_id));
joinable!(comments -> app (app_id));
joinable!(comments -> locale (locale_id));
joinable!(keywords -> app (app_id));
joinable!(keywords -> locale (locale_id));

allow_tables_to_appear_in_same_query!(app, app_locale, comments, keywords, locale,);
