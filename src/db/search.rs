use diesel::debug_query;
use diesel::sqlite::Sqlite;
use crate::diesel::JoinOnDsl;

use diesel::{
    BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl,
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
    ) -> Result<Vec<SearchResult>, diesel::result::Error>;
}

impl SearchDb for DesktopDDb {
    fn get(
        &mut self,
        text: &str,
        locale: &str,
        limit: u8,
    ) -> Result<Vec<SearchResult>, diesel::result::Error> {
        use crate::schema::{app, app_locale, locale, comments, keywords};

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
            .inner_join(
                app_locale::dsl::app_locale.on(app_locale::app_id.eq(app::id))
            )
            .inner_join(
                locale::dsl::locale.on(locale::id.eq(app_locale::locale_id))
            )
            .inner_join(keywords::dsl::keywords)
            .inner_join(comments::dsl::comments.on(
                    comments::app_id.eq(app::id)
                    .and(
                        comments::locale_id.eq(locale::id)
                    )
                )
            )
            .filter(
                keywords::key
                    .like(format!("%{}%", text))
                    .and(
                        locale::key.eq(lang)
                        .or(
                            locale::key.eq(location)
                        )
                        .or(
                            locale::key.eq("__NO_LOCALE__")
                        )
                    )
            )
            //.limit(limit.into())
            .select(selection)
            .group_by(selection)
            ;

        if self.debug {
            let sql_debug = debug_query::<Sqlite, _>(&query);
            println!("{}", sql_debug);
        }

        query.load::<SearchResult>(&mut self.connection)
    }
}
