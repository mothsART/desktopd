use crate::schema::app;

#[derive(Insertable)]
#[table_name="app"]
pub struct NewApp<'a> {
    pub title: &'a str,
    pub path: &'a str,

    pub generic_title: Option<&'a str>,
    pub exec: Option<&'a str>,
    pub try_exec: Option<&'a str>,
    pub icon_path: Option<&'a str>,
    pub lang: Option<&'a str>,
}
