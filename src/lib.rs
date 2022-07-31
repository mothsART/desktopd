#[macro_use]
extern crate diesel;

pub mod collect;
pub mod desktop;
pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel::Connection;
use diesel::sqlite::SqliteConnection;
use diesel::result::Error;

use crate::desktop::DesktopFile;
use self::models::NewApp;

pub struct DesktopDDb {
    pub connection: SqliteConnection,
}

impl DesktopDDb {
    pub fn new() -> DesktopDDb {
        let database_url = "desktopd.db";
        let connection = SqliteConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
        DesktopDDb {
            connection,
        }
    }

    pub fn insertion(&self, desktop_files: Vec<DesktopFile>) {
        use schema::app;

        self.connection.transaction::<_, Error, _>(|| {

            diesel::delete(app::table).execute(&self.connection)?;

            for d in desktop_files {
                let default_app = NewApp {
                    title: &d.default_name,
                    path: &d.path,
                    generic_title: d.default_generic_name.as_ref().map(String::as_str),
                    exec: d.exec.as_ref().map(String::as_str),
                    try_exec: d.try_exec.as_ref().map(String::as_str),
                    icon_path: d.icon.as_ref().map(String::as_str),
                    lang: None,
                };

                diesel::insert_into(app::table)
                    .values(&default_app)
                    .execute(&self.connection)?;

                for r in d.i18n_names {
                    let lang_app = NewApp {
                        title: &r.1,
                        path: &d.path,
                        generic_title: d.default_generic_name.as_ref().map(String::as_str),
                        exec: d.exec.as_ref().map(String::as_str),
                        try_exec: d.try_exec.as_ref().map(String::as_str),
                        icon_path: d.icon.as_ref().map(String::as_str),
                        lang: Some(&r.0),
                    };

                    diesel::insert_into(app::table)
                    .values(&lang_app)
                    .execute(&self.connection)?;
                }
                //break;
            }
            Ok(())
        });
    }

    pub fn delete(&self, desktop_file: DesktopFile) {
        
    }
}
