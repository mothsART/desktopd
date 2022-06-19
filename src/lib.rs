#[macro_use]
extern crate diesel;

pub mod collect;
pub mod desktop;
pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel::Connection;
use diesel::sqlite::SqliteConnection;

use crate::desktop::DesktopFile;
use self::models::NewApp;

pub fn insertion(desktop_file: DesktopFile) {
    use schema::app;

    let new_app = NewApp {
        title: &desktop_file.default_name,
        path: &desktop_file.path,

        generic_title: desktop_file.default_generic_name.as_ref().map(String::as_str),
        exec: desktop_file.exec.as_ref().map(String::as_str),
        try_exec: desktop_file.try_exec.as_ref().map(String::as_str),
        icon_path: desktop_file.icon.as_ref().map(String::as_str),
    };

    let database_url = "desktopd.db";
    let conn = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    println!("{:?} {:?}", new_app.path, new_app.title);
    diesel::insert_into(app::table)
        .values(&new_app)
        .execute(&conn)
        .expect("Error saving new post");
}
