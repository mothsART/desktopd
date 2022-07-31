#[macro_use]
extern crate diesel;

use std::env;

use std::path::Path;

mod collect;
mod desktop;
mod watch;
mod models;
mod schema;

use desktopd::desktop::DesktopFile;
use desktopd::DesktopDDb;

use crate::collect::{collect_desktops_path, get_dirs};
use crate::watch::watch;

fn main() {
    let xdg_data_dirs = env::var("XDG_DATA_DIRS");
    let db = DesktopDDb::new();

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
}
