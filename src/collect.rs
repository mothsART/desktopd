use std::path::Path;

use glob::glob;
use itertools::Itertools;

pub fn get_dirs(xdg_data_dirs: &str) -> Vec<String> {
    xdg_data_dirs
        .split(':')
        .filter(|&s| Path::new(s).is_dir())
        .map(|s| {
            Path::new(s)
                .join("applications")
                .to_str()
                .unwrap()
                .to_string()
        })
        .filter(|s| Path::new(s).is_dir())
        .dedup()
        .collect()
}

pub fn collect_desktops_path(dirs: Vec<String>) -> Vec<String> {
    let mut result = vec![];
    for d in dirs {
        for desktop_file in glob(&format!("{d}/*.desktop")).unwrap() {
            match desktop_file {
                Ok(path) => result.push(path.display().to_string()),
                Err(_e) => continue,
            }
        }
    }
    result
}
