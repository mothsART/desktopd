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
            app::exec,
            app::try_exec,
            app::icon_path,
            comments::title,
        );

        allow_columns_to_appear_in_same_group_by_clause!(
            app::title,
            app::path,
            app::generic_title,
            app::exec,
            app::try_exec,
            app::icon_path,
            comments::title,
        );

        // TODO : I'm not sure, "en" was the best choice
        let lang = locale.get(0..2).unwrap_or("en");
        let location = locale.get(0..5).unwrap_or("en_EN");

        let query = app::dsl::app
            .left_join(keywords::dsl::keywords)
            .inner_join(comments::dsl::comments)
            .filter(
                app::title
                    .like(format!("{}%", text))
                    .or(
                        comments::title
                            .like(format!("%{}%", text))
                            .and(
                                comments::lang.eq(lang)
                                .or(
                                    comments::lang.eq(location)
                                )
                            )
                    )
                    .or(
                        keywords::key
                            .like(format!("{}%", text))
                            .and(
                                keywords::lang.eq(lang)
                                .or(
                                    keywords::lang.eq(location)
                                )
                            )
                    )
            )
            //.limit(limit.into())
            .select(selection.nullable())
            .group_by(selection);

        if self.debug {
            let sql_debug = debug_query::<Sqlite, _>(&query);
            println!("{}", sql_debug);
        }

        query.load::<Option<SearchResult>>(&mut self.connection)
    }
}
