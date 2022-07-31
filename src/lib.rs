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
use self::models::{NewApp, NewNames, NewGenericNames, NewComments, NewKeywords};

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
        use schema::{app, names, generic_names, comments, keywords};

        self.connection.transaction::<_, Error, _>(|| {

            diesel::delete(app::table).execute(&self.connection)?;
            diesel::delete(names::table).execute(&self.connection)?;

            for d in desktop_files {
                let default_app = NewApp {
                    title: &d.default_name,
                    path: &d.path,
                    generic_title: d.default_generic_name.as_ref().map(String::as_str),
                    exec: d.exec.as_ref().map(String::as_str),
                    try_exec: d.try_exec.as_ref().map(String::as_str),
                    icon_path: d.icon.as_ref().map(String::as_str),
                };

                let app_id = diesel::insert_into(app::table)
                    .values(&default_app)
                    .execute(&self.connection)?;

                for r in d.i18n_names {
                    let lang_app = NewNames {
                        title: &r.1,
                        app_id: app_id as i32,
                        lang: &r.0,
                    };

                    diesel::insert_into(names::table)
                    .values(&lang_app)
                    .execute(&self.connection)?;
                }
                break;
            }
            Ok(())
        });
    }

    pub fn delete(&self, desktop_file: DesktopFile) {
        
    }
}
