use std::env;

mod collect;

use crate::collect::{collect_desktops_path, get_dirs};

fn main() {
    let xdg_data_dirs = env::var("XDG_DATA_DIRS");

    match xdg_data_dirs {
        Ok(v) => {
            let dirs = get_dirs(&v);
            println!("{:?}", dirs);
            println!("{:?}", collect_desktops_path(dirs));
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
