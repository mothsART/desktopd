use diesel::debug_query;
use diesel::sqlite::Sqlite;

use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, TextExpressionMethods, BoolExpressionMethods};

use crate::db::basic::DesktopDDb;
use crate::desktop::DesktopFile;
use crate::models::{App, Comments, Keywords};

pub trait SearchDb {
    fn get(&self, text: &str, locale: &str); //-> Result<Vec<(App, Option<Keywords>, Option<Comments>)>, diesel::result::Error>;
}

impl SearchDb for DesktopDDb {
    fn get(&self, text: &str, locale: &str) { // -> Result<Vec<(App, Option<Keywords>, Option<Comments>)>, diesel::result::Error> {
        use crate::schema::app::dsl::app;

        use crate::schema::app::{
            id as app_id,
            title as a_title,
        };
        use crate::schema::comments::{
            app_id as c_app_id,
            title as c_title,
            lang as c_lang,
        };
        use crate::schema::keywords::{
            app_id as k_app_id,
            key,
            lang as k_lang,
        };

        // TODO : I'm not sure, "en" was the best choice
        let lang = locale.get(0..2).unwrap_or("en");

        let query = app
            .left_join(crate::schema::keywords::table.on(k_app_id.eq(crate::schema::app::id)))
            .left_join(crate::schema::comments::table.on(c_app_id.eq(app_id)))
            .filter(
                c_title.like(format!("%{}%", text))
                .and(c_lang.like(format!("{}%", lang)))
            )
            .or_filter(
                key.like(format!("{}%", text))
                .and(
                    k_lang.like(format!("{}%", lang))
                    .or(k_lang.is_null())
                )
            )
            .or_filter(a_title.like(format!("{}%", text)))
            .group_by(&app_id)
            //.select((app_id, a_title))
            ;

        let debug = debug_query::<Sqlite, _>(&query);
        println!("{}", debug.to_string());
        //let results = query.load::<(App, Option<Keywords>, Option<Comments>)>(&self.connection);
        //let results = query.load::<(i32, String)>(&mut self.connection);
        //println!("{:?}", results);
        //println!("{:?}", results.expect("REASON").len());
    }
}
