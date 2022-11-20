#[macro_use]
extern crate diesel;

use std::env;

use std::path::Path;

mod collect;
mod desktop;
mod models;
mod schema;
mod watch;

use desktopd::db::basic::{Db, DesktopDDb};
use desktopd::db::populate::PopulateDb;
use desktopd::desktop::DesktopFile;

use crate::collect::{collect_desktops_path, get_dirs};
use crate::watch::watch;

fn main() {
    let xdg_data_dirs = env::var("XDG_DATA_DIRS");
    let mut db = DesktopDDb::new();

    match xdg_data_dirs {
        Ok(v) => {
            let dirs = get_dirs(&v);
            let mut desktop_files = vec![];
            for path in collect_desktops_path(dirs) {
                let d = DesktopFile::new(Path::new(&path));
                desktop_files.push(d);
            }
            db.insertion(desktop_files);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    watch();
}
