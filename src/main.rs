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
use desktopd::insertion;
use crate::collect::{collect_desktops_path, get_dirs};
use crate::watch::watch;

fn main() {

    let xdg_data_dirs = env::var("XDG_DATA_DIRS");

    match xdg_data_dirs {
        Ok(v) => {
            let dirs = get_dirs(&v);
            
            for path in collect_desktops_path(dirs) {
                let d = DesktopFile::new(Path::new(&path));
                insertion(d);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
