use crate::schema::{app, comments, keywords};

#[derive(Debug, Queryable)]
pub struct SearchResult {
    pub title: String,
    pub path: String,
    pub generic_title: Option<String>,
    pub exec: String,
    pub try_exec: Option<String>,
    pub icon_path: Option<String>,

    pub comment:  Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = app)]
pub struct NewApp<'a> {
    pub title: &'a str,
    pub path: &'a str,
    pub generic_title: Option<&'a str>,
    pub exec: &'a str,
    pub try_exec: Option<&'a str>,
    pub icon_path: Option<&'a str>,
}

#[derive(Insertable)]
#[diesel(table_name = comments)]
pub struct NewComments<'a> {
    pub title: &'a str,
    pub app_id: i32,
    pub lang: Option<&'a str>,
}

#[derive(Insertable)]
#[diesel(table_name = keywords)]
pub struct NewKeywords<'a> {
    pub key: &'a str,
    pub app_id: i32,
    pub lang: Option<&'a str>,
}
