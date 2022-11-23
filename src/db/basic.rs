use std::env;

use crate::diesel::Connection;
use diesel::SqliteConnection;
use dotenvy::dotenv;

pub struct DesktopDDb {
    pub debug: bool,
    pub connection: SqliteConnection,
}

pub trait Db {
    fn new(debug: bool) -> Self;
}

impl Db for DesktopDDb {
    fn new(debug: bool) -> Self {
        dotenv().ok();
        // https://serverfault.com/questions/413397/how-to-set-environment-variable-in-systemd-service
        let database_path = env::var("DESKTOPD_DB_PATH").expect("DESKTOPD_DB_PATH must be set");
        let connection = SqliteConnection::establish(&database_path)
            .unwrap_or_else(|_| panic!("Error connecting to {}", &database_path));
        DesktopDDb {
            debug,
            connection
        }
    }
}
