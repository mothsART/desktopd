use crate::schema::{app, names, generic_names, comments, keywords};

#[derive(Insertable)]
#[table_name="app"]
pub struct NewApp<'a> {
    pub title: &'a str,
    pub path: &'a str,
    pub generic_title: Option<&'a str>,
    pub exec: Option<&'a str>,
    pub try_exec: Option<&'a str>,
    pub icon_path: Option<&'a str>,
}

#[derive(Insertable)]
#[table_name="names"]
pub struct NewNames<'a> {
    pub title: &'a str,
    pub app_id: i32,
    pub lang: &'a str,
}

#[derive(Insertable)]
#[table_name="generic_names"]
pub struct NewGenericNames<'a> {
    pub title: &'a str,
    pub app_id: i32,
    pub lang: Option<&'a str>,
}

#[derive(Insertable)]
#[table_name="comments"]
pub struct NewComments<'a> {
    pub title: &'a str,
    pub app_id: i32,
    pub lang: Option<&'a str>,
}

#[derive(Insertable)]
#[table_name="keywords"]
pub struct NewKeywords<'a> {
    pub key: &'a str,
    pub app_id: i32,
    pub lang: Option<&'a str>,
}
