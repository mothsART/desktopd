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

                for n in d.i18n_names {
                    let lang_app = NewNames {
                        title: &n.1,
                        app_id: app_id as i32,
                        lang: &n.0,
                    };

                    diesel::insert_into(names::table)
                    .values(&lang_app)
                    .execute(&self.connection)?;
                }

                for g in d.i18n_generic_names {
                    let generic_name = NewGenericNames {
                        title: &g.1,
                        app_id: app_id as i32,
                        lang: &g.0,
                    };
                    diesel::insert_into(generic_names::table)
                        .values(&generic_name)
                        .execute(&self.connection)?;
                }

                for c in d.i18n_comments {
                    let comment = NewComments {
                        title: &c.1,
                        app_id: app_id as i32,
                        lang: &c.0,
                    };
                    diesel::insert_into(comments::table)
                        .values(&comment)
                        .execute(&self.connection)?;
                }

                for k in d.default_keywords {
                    let keyword = NewKeywords {
                        key: &k,
                        app_id: app_id as i32,
                        lang: None,
                    };
                    diesel::insert_into(keywords::table)
                        .values(&keyword)
                        .execute(&self.connection)?;
                }

                for k_lang in d.i18n_keywords {
                    for k in k_lang.1 {
                        let keyword = NewKeywords {
                            key: &k,
                            app_id: app_id as i32,
                            lang: Some(&k_lang.0),
                        };
                        diesel::insert_into(keywords::table)
                            .values(&keyword)
                            .execute(&self.connection)?;
                    }
                }
            }
            Ok(())
        });
    }

    pub fn delete(&self, desktop_file: DesktopFile) {
        
    }
}
