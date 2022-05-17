use std::env;
use std::path::Path;

use itertools::Itertools;
use glob::glob;

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
        .into_iter()
        .dedup()
        .collect()
}

pub fn collect_desktops_path(dirs: Vec<String>) -> Vec<String> {
    let mut result = vec![];
    for d in dirs {
        for desktop_file in glob(&format!("{}/*.desktop", d)).unwrap() {
            match desktop_file {
                Ok(path) => result.push(path.display().to_string()),
                Err(_e) => continue,
            }
        }
    }
    result
}

#[cfg(target_os = "linux")]
#[cfg(test)]
mod tests {
    use super::{get_dirs, collect_desktops_path};
    use fs_extra::dir::{copy, CopyOptions};
    use tempfile::{tempdir, TempDir};

    fn fake_xdg_data_env(tmp_dir: &TempDir, dirs: Vec<&str>) -> String {
        let mut result = String::new();
        for dir in dirs {
            result.push_str(&format!(
                "{}:",
                tmp_dir.path().join("test1").join(dir).display()
            ));
        }
        result.remove(result.len() - 1);
        result
    }

    /// Create this tree :
    /// /tmp/generate-dir
    ///    |
    ///    └── path-1/applications/app-one.desktop
    ///    └── path-1/applications/app-two.desktop
    ///    └── path-2/applications (without .desktop files)
    ///    └── path-3 (without applications dirs)
    ///    └── path-4/applications/app-one.desktop
    ///    └── path-4/applications/app-two.desktop (blank file)
    ///    └── path-4/applications/app-three.desktop (corrupt file)
    ///    └── path-4/applications/app-four.desktop (loked file)
    ///    └── path-4/applications/app-six.desktop (wrong rights file)
    #[test]
    fn test_dirs() {
        let options = CopyOptions::new();
        let tmp_dir = tempdir().unwrap();
        copy("tests/test1", &tmp_dir, &options).unwrap();

        let env = fake_xdg_data_env(
            &tmp_dir,
            vec!["path-1", "path-2", "path-3", "path-4", "path-5", "path-4"],
        );
        let dirs = get_dirs(&env);
        let mut result: Vec<String> = vec![];
        for d in dirs {
            result.push(d[15..].to_string());
        }
        assert_eq!(
            result,
            vec![
                "/test1/path-1/applications",
                "/test1/path-2/applications",
                "/test1/path-4/applications",
            ]
        );
        tmp_dir.close().unwrap();
    }

    /// Create this tree :
    /// /tmp/generate-dir
    ///    |
    ///    └── path-1/applications/app-one.desktop
    ///    └── path-1/applications/app-two.desktop
    ///    └── path-2/applications (without .desktop files)
    ///    └── path-3 (without applications dirs)
    ///    └── path-4/applications/app-one.desktop
    ///    └── path-4/applications/app-two.desktop (blank file)
    ///    └── path-4/applications/app-three.desktop (corrupt file)
    ///    └── path-4/applications/app-four.desktop (loked file)
    ///    └── path-4/applications/app-six.desktop (wrong rights file)
    #[test]
    fn test_desktops_path() {
        let options = CopyOptions::new();
        let tmp_dir = tempdir().unwrap();
        copy("tests/test1", &tmp_dir, &options).unwrap();

        let env = fake_xdg_data_env(
            &tmp_dir,
            vec!["path-1", "path-2", "path-3", "path-4", "path-5", "path-4"],
        );
        let desktop_paths = collect_desktops_path(get_dirs(&env));
        let mut result: Vec<String> = vec![];
        for d in desktop_paths {
            result.push(d[15..].to_string());
        }
        assert_eq!(
            result,
            vec![
                "/test1/path-1/applications/app-one.desktop",
                "/test1/path-1/applications/app-two.desktop",
                "/test1/path-4/applications/app-four.desktop",
                "/test1/path-4/applications/app-one.desktop",
                "/test1/path-4/applications/app-six.desktop",
                "/test1/path-4/applications/app-three.desktop",
                "/test1/path-4/applications/app-two.desktop",
            ]
        );
        tmp_dir.close().unwrap();
    }
}

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
