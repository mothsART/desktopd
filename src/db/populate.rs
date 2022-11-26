use crate::diesel::{Connection, RunQueryDsl};
use diesel::result::Error;

use crate::db::basic::DesktopDDb;
use crate::desktop::DesktopFile;
use crate::models::{NewApp, NewComments, NewKeywords};

pub trait PopulateDb {
    fn insertion(&mut self, desktop_files: Vec<DesktopFile>);
}

impl PopulateDb for DesktopDDb {
    fn insertion(&mut self, desktop_files: Vec<DesktopFile>) {
        use crate::schema::{app, comments, keywords};

        let _result = self.connection.transaction::<_, Error, _>(|connection| {
            diesel::delete(app::table).execute(connection)?;
            diesel::delete(comments::table).execute(connection)?;
            diesel::delete(keywords::table).execute(connection)?;

            let mut app_id = 0;
            for d in desktop_files {
                if let Some(exec) = d.exec.as_deref() {
                    let default_app = NewApp {
                        title: &d.default_name,
                        path: &d.path,
                        generic_title: d.default_generic_name.as_deref(),
                        exec: exec,
                        try_exec: d.try_exec.as_deref(),
                        icon_path: d.icon.as_deref(),
                    };

                    diesel::insert_into(app::table)
                        .values(&default_app)
                        .execute(connection)?;

                    app_id += 1;

                    for n in d.i18n_names {
                        let keyword = NewKeywords {
                            key: &n.1,
                            app_id: app_id as i32,
                            lang: Some(&n.0),
                        };
                        diesel::insert_into(keywords::table)
                            .values(&keyword)
                            .execute(connection)?;
                    }

                    for g in d.i18n_generic_names {
                        let keyword = NewKeywords {
                            key: &g.1,
                            app_id: app_id as i32,
                            lang: Some(&g.0),
                        };
                        diesel::insert_into(keywords::table)
                            .values(&keyword)
                            .execute(connection)?;
                    }

                    if let Some(d_comment) = d.default_comment.as_deref() {
                        let default_comment = NewComments {
                            title: d_comment,
                            app_id: app_id as i32,
                            lang: None,
                        };
                        diesel::insert_into(comments::table)
                            .values(&default_comment)
                            .execute(connection)?;
                    }

                    for c in d.i18n_comments {
                        let comment = NewComments {
                            title: &c.1,
                            app_id: app_id as i32,
                            lang: Some(&c.0),
                        };
                        diesel::insert_into(comments::table)
                            .values(&comment)
                            .execute(connection)?;
                    }

                    for k in d.default_keywords {
                        let keyword = NewKeywords {
                            key: &k,
                            app_id: app_id as i32,
                            lang: None,
                        };
                        diesel::insert_into(keywords::table)
                            .values(&keyword)
                            .execute(connection)?;
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
                                .execute(connection)?;
                        }
                    }
                }
            }
            Ok(())
        });
    }
}
