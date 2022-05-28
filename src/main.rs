use std::env;

use std::path::Path;

mod collect;
mod desktop;

use crate::collect::{collect_desktops_path, get_dirs};
use crate::desktop::DesktopFile;

fn main() {
    let d = DesktopFile::new(Path::new("tests/test1/path-1/applications/app-one.desktop"));
    println!("{:?}", d);
    /*
    let xdg_data_dirs = env::var("XDG_DATA_DIRS");

    match xdg_data_dirs {
        Ok(v) => {
            let dirs = get_dirs(&v);
            println!("{:?}", dirs);
            println!("{:?}", collect_desktops_path(dirs));
        }
        Err(e) => eprintln!("Error: {}", e),
    }*/
}
