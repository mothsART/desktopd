use std::collections::{HashMap, HashSet};

use crate::diesel::{Connection, RunQueryDsl};
use diesel::result::Error;

use crate::db::basic::DesktopDDb;
use crate::desktop::DesktopFile;
use crate::models::{NewApp, NewAppLocale, NewComments, NewKeywords, NewLocale};

pub trait PopulateDb {
    fn insertion(&mut self, desktop_files: Vec<DesktopFile>);
}

impl PopulateDb for DesktopDDb {
    fn insertion(&mut self, desktop_files: Vec<DesktopFile>) {
        use crate::schema::{app, app_locale, comments, keywords, locale};

        let _result = self.connection.transaction::<_, Error, _>(|connection| {
            diesel::delete(app::table).execute(connection)?;
            diesel::delete(app_locale::table).execute(connection)?;
            diesel::delete(locale::table).execute(connection)?;
            diesel::delete(comments::table).execute(connection)?;
            diesel::delete(keywords::table).execute(connection)?;

            let mut locale_id = 1;
            let mut locales = HashMap::new();

            for d in &desktop_files {
                if let Some(_exec) = d.exec.as_deref() {
                    for c in &d.i18n_comments {
                        if locales.contains_key(c.0) {
                            continue;
                        }
                        locales.insert(c.0.clone(), locale_id);
                        let l = NewLocale { key: &c.0 };
                        diesel::insert_into(locale::table)
                            .values(&l)
                            .execute(connection)?;
                        locale_id += 1;
                    }

                    for k_lang in &d.i18n_keywords {
                        if locales.contains_key(k_lang.0) {
                            continue;
                        }
                        locales.insert(k_lang.0.clone(), locale_id);
                        let l = NewLocale { key: &k_lang.0 };
                        diesel::insert_into(locale::table)
                            .values(&l)
                            .execute(connection)?;
                        locale_id += 1;
                    }
                }
            }
            let no_locale = "__NO_LOCALE__";
            locales.insert(no_locale.to_string(), locale_id);
            let l = NewLocale { key: no_locale };
            diesel::insert_into(locale::table)
                .values(&l)
                .execute(connection)?;

            let mut app_id = 0;

            let mut constrain_app_locale = HashSet::new();

            let mut constrain_app_keywords = HashSet::new();
            let mut constrain_locale_keywords = HashSet::new();

            let mut constrain_app_comments = HashSet::new();
            let mut constrain_locale_comments = HashSet::new();

            for d in &desktop_files {
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

                    if let Some(_locale_id) = locales.get(no_locale) {
                        let app_hash = format!("{}_{}", app_id, d.default_name);
                        let locale_hash = format!("{}_{}", _locale_id, d.default_name);
                        if !constrain_app_keywords.contains(&app_hash)
                            && !constrain_locale_keywords.contains(&locale_hash)
                        {
                            constrain_app_keywords.insert(app_hash);
                            constrain_locale_keywords.insert(locale_hash);

                            let a_l_hash = format!("{}_{}", app_id, _locale_id);
                            if !constrain_app_locale.contains(&a_l_hash) {
                                constrain_app_locale.insert(a_l_hash);

                                let a_l = NewAppLocale {
                                    app_id: app_id as i32,
                                    locale_id: *_locale_id,
                                };
                                diesel::insert_into(app_locale::table)
                                    .values(&a_l)
                                    .execute(connection)?;
                            }

                            let default_comment = NewKeywords {
                                key: &d.default_name,
                                app_id: app_id as i32,
                                locale_id: *_locale_id,
                            };
                            diesel::insert_into(keywords::table)
                                .values(&default_comment)
                                .execute(connection)?;
                        }
                    }

                    for n in &d.i18n_names {
                        if let Some(_locale_id) = locales.get(n.0) {
                            let app_hash = format!("{}_{}", app_id, n.1);
                            let locale_hash = format!("{}_{}", _locale_id, n.1);
                            if constrain_app_keywords.contains(&app_hash)
                                || constrain_locale_keywords.contains(&locale_hash)
                            {
                                continue;
                            }
                            constrain_app_keywords.insert(app_hash);
                            constrain_locale_keywords.insert(locale_hash);

                            let a_l_hash = format!("{}_{}", app_id, _locale_id);
                            if !constrain_app_locale.contains(&a_l_hash) {
                                constrain_app_locale.insert(a_l_hash);

                                let a_l = NewAppLocale {
                                    app_id: app_id as i32,
                                    locale_id: *_locale_id,
                                };
                                diesel::insert_into(app_locale::table)
                                    .values(&a_l)
                                    .execute(connection)?;
                            }

                            let keyword = NewKeywords {
                                key: &n.1,
                                app_id: app_id as i32,
                                locale_id: *_locale_id,
                            };
                            diesel::insert_into(keywords::table)
                                .values(&keyword)
                                .execute(connection)?;
                        }
                    }

                    for g in &d.i18n_generic_names {
                        if let Some(_locale_id) = locales.get(g.0) {
                            let app_hash = format!("{}_{}", app_id, g.1);
                            let locale_hash = format!("{}_{}", _locale_id, g.1);
                            if constrain_app_keywords.contains(&app_hash)
                                || constrain_locale_keywords.contains(&locale_hash)
                            {
                                continue;
                            }
                            constrain_app_keywords.insert(app_hash);
                            constrain_locale_keywords.insert(locale_hash);

                            let a_l_hash = format!("{}_{}", app_id, _locale_id);
                            if !constrain_app_locale.contains(&a_l_hash) {
                                constrain_app_locale.insert(a_l_hash);

                                let a_l = NewAppLocale {
                                    app_id: app_id as i32,
                                    locale_id: *_locale_id,
                                };
                                diesel::insert_into(app_locale::table)
                                    .values(&a_l)
                                    .execute(connection)?;
                            }

                            let keyword = NewKeywords {
                                key: &g.1,
                                app_id: app_id as i32,
                                locale_id: *_locale_id,
                            };
                            diesel::insert_into(keywords::table)
                                .values(&keyword)
                                .execute(connection)?;
                        }
                    }

                    if let (Some(d_comment), Some(_locale_id)) =
                        (d.default_comment.as_deref(), locales.get(no_locale))
                    {
                        let app_hash = format!("{}_{}", app_id, d_comment);
                        let locale_hash = format!("{}_{}", _locale_id, d_comment);
                        if !constrain_app_comments.contains(&app_hash)
                            && !constrain_locale_comments.contains(&locale_hash)
                        {
                            constrain_app_comments.insert(app_hash);
                            constrain_locale_comments.insert(locale_hash);

                            let a_l_hash = format!("{}_{}", app_id, _locale_id);
                            if !constrain_app_locale.contains(&a_l_hash) {
                                constrain_app_locale.insert(a_l_hash);

                                let a_l = NewAppLocale {
                                    app_id: app_id as i32,
                                    locale_id: *_locale_id,
                                };
                                diesel::insert_into(app_locale::table)
                                    .values(&a_l)
                                    .execute(connection)?;
                            }

                            let default_comment = NewComments {
                                title: d_comment,
                                app_id: app_id as i32,
                                locale_id: *_locale_id,
                            };
                            diesel::insert_into(comments::table)
                                .values(&default_comment)
                                .execute(connection)?;
                        }
                    }

                    for c in &d.i18n_comments {
                        if let Some(_locale_id) = locales.get(c.0) {
                            let app_hash = format!("{}_{}", app_id, c.1);
                            let locale_hash = format!("{}_{}", _locale_id, c.1);
                            if constrain_app_comments.contains(&app_hash)
                                || constrain_locale_comments.contains(&locale_hash)
                            {
                                continue;
                            }
                            constrain_app_comments.insert(app_hash);
                            constrain_locale_comments.insert(locale_hash);

                            let a_l_hash = format!("{}_{}", app_id, _locale_id);
                            if !constrain_app_locale.contains(&a_l_hash) {
                                constrain_app_locale.insert(a_l_hash);

                                let a_l = NewAppLocale {
                                    app_id: app_id as i32,
                                    locale_id: *_locale_id,
                                };
                                diesel::insert_into(app_locale::table)
                                    .values(&a_l)
                                    .execute(connection)?;
                            }

                            let comment = NewComments {
                                title: &c.1,
                                app_id: app_id as i32,
                                locale_id: *_locale_id,
                            };
                            diesel::insert_into(comments::table)
                                .values(&comment)
                                .execute(connection)?;
                        }
                    }

                    for k in &d.default_keywords {
                        if let Some(_locale_id) = locales.get(no_locale) {
                            let app_hash = format!("{}_{}", app_id, k);
                            let locale_hash = format!("{}_{}", _locale_id, k);
                            if constrain_app_keywords.contains(&app_hash)
                                || constrain_locale_keywords.contains(&locale_hash)
                            {
                                continue;
                            }
                            constrain_app_keywords.insert(app_hash);
                            constrain_locale_keywords.insert(locale_hash);

                            let a_l_hash = format!("{}_{}", app_id, _locale_id);
                            if !constrain_app_locale.contains(&a_l_hash) {
                                constrain_app_locale.insert(a_l_hash);

                                let a_l = NewAppLocale {
                                    app_id: app_id as i32,
                                    locale_id: *_locale_id,
                                };
                                diesel::insert_into(app_locale::table)
                                    .values(&a_l)
                                    .execute(connection)?;
                            }

                            let keyword = NewKeywords {
                                key: &k,
                                app_id: app_id as i32,
                                locale_id: *_locale_id,
                            };
                            diesel::insert_into(keywords::table)
                                .values(&keyword)
                                .execute(connection)?;
                        }
                    }

                    for k_lang in &d.i18n_keywords {
                        if let Some(_locale_id) = locales.get(k_lang.0) {
                            for k in k_lang.1 {
                                let app_hash = format!("{}_{}", app_id, k);
                                let locale_hash = format!("{}_{}", _locale_id, k);
                                if constrain_app_keywords.contains(&app_hash)
                                    || constrain_locale_keywords.contains(&locale_hash)
                                {
                                    continue;
                                }
                                constrain_app_keywords.insert(app_hash);
                                constrain_locale_keywords.insert(locale_hash);

                                let a_l_hash = format!("{}_{}", app_id, _locale_id);
                                if !constrain_app_locale.contains(&a_l_hash) {
                                    constrain_app_locale.insert(a_l_hash);

                                    let a_l = NewAppLocale {
                                        app_id: app_id as i32,
                                        locale_id: *_locale_id,
                                    };
                                    diesel::insert_into(app_locale::table)
                                        .values(&a_l)
                                        .execute(connection)?;
                                }

                                let keyword = NewKeywords {
                                    key: &k,
                                    app_id: app_id as i32,
                                    locale_id: 0,
                                };
                                diesel::insert_into(keywords::table)
                                    .values(&keyword)
                                    .execute(connection)?;
                            }
                        }
                    }
                }
            }
            Ok(())
        });
    }
}
