use std::env;
use std::env::VarError;
use std::path::Path;

use itertools::Itertools;

fn get_dirs(env_name: &str) -> Result<Vec<String>, VarError> {
    let xdg_data_dirs = env::var(env_name);
    let split: Vec<String> = xdg_data_dirs?.split(':')
        .filter(|&s| Path::new(s).is_dir())
        .map(|s| Path::new(s).join("applications").to_str().unwrap().to_string())
        .filter(|s| Path::new(s).is_dir())
        .into_iter()
        .dedup()
        .collect();
    Ok(split)
}

fn main() {
    let xdg_data_dirs = get_dirs("XDG_DATA_DIRS");

    match xdg_data_dirs {
        Ok(v) => println!("{:?}", v),
        Err(e) => eprintln!("Error: {}", e),
    }
}
