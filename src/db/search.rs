use diesel::debug_query;
use diesel::sqlite::Sqlite;

use diesel::{
    BoolExpressionMethods, ExpressionMethods, NullableExpressionMethods, QueryDsl, RunQueryDsl,
    TextExpressionMethods,
};

use crate::db::basic::DesktopDDb;
use crate::models::SearchResult;

pub trait SearchDb {
    fn get(
        &mut self,
        text: &str,
        locale: &str,
        limit: u8,
    ) -> Result<Vec<Option<SearchResult>>, diesel::result::Error>;
}

impl SearchDb for DesktopDDb {
    fn get(
        &mut self,
        text: &str,
        locale: &str,
        limit: u8,
    ) -> Result<Vec<Option<SearchResult>>, diesel::result::Error> {
        use crate::schema::{app, comments, keywords};

        let selection = (
            app::title,
            app::path,
            app::generic_title,
            app::comment,
            app::exec,
            app::try_exec,
            app::icon_path,
            comments::title,
        );

        allow_columns_to_appear_in_same_group_by_clause!(
            app::title,
            app::path,
            app::generic_title,
            app::comment,
            app::exec,
            app::try_exec,
            app::icon_path,
            comments::title,
        );

        // TODO : I'm not sure, "en" was the best choice
        let lang = locale.get(0..2).unwrap_or("en");

        let query = app::dsl::app
            .left_join(keywords::dsl::keywords)
            .left_join(comments::dsl::comments)
            .filter(
                keywords::lang
                    .like(format!("{}%", lang))
                    .and(comments::lang.like(format!("{}%", lang)))
                    .and(
                        comments::title
                            .like(format!("%{}%", text))
                            .or(keywords::key
                                .like(format!("{}%", text))
                                .or(keywords::lang.is_null()))
                            .or(app::title.like(format!("{}%", text))),
                    ),
            )
            .limit(limit.into())
            .select(selection.nullable())
            .group_by(selection);

        if self.debug {
            let sql_debug = debug_query::<Sqlite, _>(&query);
            println!("{}", sql_debug);
        }

        query.load::<Option<SearchResult>>(&mut self.connection)
    }
}
