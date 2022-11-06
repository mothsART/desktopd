use std::env;

use crate::diesel::Connection;
use dotenvy::dotenv;
use diesel::SqliteConnection;

pub struct DesktopDDb {
    pub connection: SqliteConnection,
}

pub trait Db {
    fn new() -> Self;
}

impl Db for DesktopDDb {
    fn new() -> Self {
        dotenv().ok();
        // https://serverfault.com/questions/413397/how-to-set-environment-variable-in-systemd-service
        let database_path = env::var("DESKTOPD_DB_PATH").expect("DESKTOPD_DB_PATH must be set");
        let connection = SqliteConnection::establish(&database_path)
            .unwrap_or_else(|_| panic!("Error connecting to {}", &database_path));
        DesktopDDb { connection }
    }
}
