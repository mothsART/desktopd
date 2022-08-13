use diesel::QueryDsl;
use diesel::JoinOnDsl;
use diesel::debug_query;
use diesel::{RunQueryDsl, Connection};
use diesel::sqlite::Sqlite;
use diesel::TextExpressionMethods;
use diesel::sql_types::Text;
use diesel::sql_query;
use diesel::associations::HasTable;
use diesel::ExpressionMethods;

use crate::desktop::DesktopFile;
use crate::db::basic::DesktopDDb;
use crate::models::{App, Keywords, Comments};

pub trait SearchDb {
    fn get(&self, text: &str) -> Vec<DesktopFile>;
}

impl SearchDb for DesktopDDb {
    fn get(&self, text: &str) -> Vec<DesktopFile> {
        use crate::schema::app::dsl::app;
        use crate::schema::keywords::dsl::keywords;

        use crate::schema::app::{id as app_id, title as a_title};
        use crate::schema::keywords::app_id as k_app_id;
        use crate::schema::comments::{app_id as c_app_id, title as c_title};

        let query = app
            .left_join(crate::schema::keywords::table.on(
                k_app_id.eq(crate::schema::app::id)
            ))
            .left_join(crate::schema::comments::table.on(
                c_app_id.eq(app_id)
            ))
            .filter(c_title.like(format!("%{}%", text)))
            .or_filter(a_title.like(format!("{}%", text)));

        let debug = debug_query::<Sqlite, _>(&query);
        println!("{}", debug.to_string());
        let results = query.load::<(App, Option<Keywords>, Option<Comments>)>(&self.connection);
        println!("{:?}", results);

        vec![]
    }
}
