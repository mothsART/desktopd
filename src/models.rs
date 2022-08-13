use crate::schema::{app, comments, keywords};

#[derive(Debug, Queryable)]
pub struct App {
    pub app_id: i32,
    pub title: String,
    pub path: String,
    pub generic_title: Option<String>,
    pub comment: Option<String>,
    pub exec: Option<String>,
    pub try_exec: Option<String>,
    pub icon_path: Option<String>,
}

#[derive(Debug, Queryable)]
pub struct Keywords {
    pub keyword_id: i32,
    pub keyword_app_id: i32,
    pub keyword_key: String,
    pub lang: Option<String>,
}

#[derive(Debug, Queryable)]
pub struct Comments {
    pub comment_id: i32,
    pub comment_app_id: i32,
    pub comment: String,
    pub lang: String,
}

#[derive(Insertable)]
#[table_name = "app"]
pub struct NewApp<'a> {
    pub title: &'a str,
    pub path: &'a str,
    pub generic_title: Option<&'a str>,
    pub exec: Option<&'a str>,
    pub try_exec: Option<&'a str>,
    pub icon_path: Option<&'a str>,
}

#[derive(Insertable)]
#[table_name = "comments"]
pub struct NewComments<'a> {
    pub title: &'a str,
    pub app_id: i32,
    pub lang: &'a str,
}

#[derive(Insertable)]
#[table_name = "keywords"]
pub struct NewKeywords<'a> {
    pub key: &'a str,
    pub app_id: i32,
    pub lang: Option<&'a str>,
}
