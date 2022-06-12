use std::env;

use std::path::Path;

mod collect;
mod desktop;
mod watch;

use crate::collect::{collect_desktops_path, get_dirs};
use crate::desktop::DesktopFile;
use crate::watch::watch;

fn main() {
    watch();
    /*
    let xdg_data_dirs = env::var("XDG_DATA_DIRS");

    match xdg_data_dirs {
        Ok(v) => {
            let dirs = get_dirs(&v);
            for path in collect_desktops_path(dirs) {
                let d = DesktopFile::new(Path::new(&path));
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    * */
}
