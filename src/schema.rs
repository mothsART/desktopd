table! {
    app (id) {
        id -> Nullable<Binary>,
        title -> Text,
        path -> Text,
        generic_title -> Nullable<Text>,
        exec -> Nullable<Text>,
        try_exec -> Nullable<Text>,
        icon_path -> Nullable<Text>,
        lang -> Nullable<Text>,
    }
}
