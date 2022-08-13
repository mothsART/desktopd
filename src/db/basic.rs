use diesel::SqliteConnection;
use crate::diesel::Connection;

pub struct DesktopDDb {
    pub connection: SqliteConnection,
}

pub trait Db {
    fn new() -> DesktopDDb;
}

impl Db for DesktopDDb {
    fn new() -> DesktopDDb {
        let database_url = "/home/jferry/projects/desktopd/desktopd.db";
        let connection = SqliteConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
        DesktopDDb {
            connection,
        }
    }
}
