use diesel::result::Error;
use crate::diesel::{RunQueryDsl, Connection};

use crate::desktop::DesktopFile;
use crate::db::basic::DesktopDDb;
use crate::models::{NewApp, NewComments, NewKeywords};

pub trait PopulateDb {
    fn insertion(&self, desktop_files: Vec<DesktopFile>);
}

use crate::diesel::QueryDsl;
use crate::diesel::query_dsl::limit_dsl::LimitDsl;

impl PopulateDb for DesktopDDb {
    fn insertion(&self, desktop_files: Vec<DesktopFile>) {
        use crate::schema::{app, comments, keywords};

        self.connection.transaction::<_, Error, _>(|| {
            diesel::delete(app::table).execute(&self.connection)?;
            diesel::delete(comments::table).execute(&self.connection)?;
            diesel::delete(keywords::table).execute(&self.connection)?;

            let mut app_id = 0;
            for d in desktop_files {
                let default_app = NewApp {
                    title: &d.default_name,
                    path: &d.path,
                    generic_title: d.default_generic_name.as_ref().map(String::as_str),
                    exec: d.exec.as_ref().map(String::as_str),
                    try_exec: d.try_exec.as_ref().map(String::as_str),
                    icon_path: d.icon.as_ref().map(String::as_str),
                };

                diesel::insert_into(app::table)
                    .values(&default_app)
                    .execute(&self.connection)?;

                app_id += 1;

                for n in d.i18n_names {
                    let keyword = NewKeywords {
                        key: &n.1,
                        app_id: app_id as i32,
                        lang: Some(&n.0),
                    };
                    diesel::insert_into(keywords::table)
                        .values(&keyword)
                        .execute(&self.connection)?;
                }

                for g in d.i18n_generic_names {
                    let keyword = NewKeywords {
                        key: &g.1,
                        app_id: app_id as i32,
                        lang: Some(&g.0),
                    };
                    diesel::insert_into(keywords::table)
                        .values(&keyword)
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
}
