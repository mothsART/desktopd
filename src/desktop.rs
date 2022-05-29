use std::collections::HashMap;
use std::path::Path;

use ini::Ini;

#[derive(Debug, PartialEq)]
pub struct DesktopFile {
    pub default_name: Option<String>,
    pub default_generic_name: Option<String>,
    pub default_comment: Option<String>,

    pub default_keywords: Vec<String>,

    pub mime_type: Vec<String>,

    pub i18n_names: HashMap<String, String>,
    pub i18n_generic_names: HashMap<String, String>,
    pub i18n_comments: HashMap<String, String>,

    pub i18n_keywords: HashMap<String, Vec<String>>,
}

fn populate_i18n(
    prefix: &'static str,
    key: String,
    value: String,
    dic: &mut HashMap<String, String>,
) -> bool {
    if !key.starts_with(prefix) {
        return false;
    }
    if let Some(lang) = key.strip_prefix(prefix).unwrap().strip_suffix(']') {
        dic.insert(lang.to_string(), value);
        return true;
    }
    false
}

impl DesktopFile {
    pub fn new(path: &Path) -> DesktopFile {
        let destktop_ini = Ini::load_from_file(path).unwrap();
        let mut i18n_names = HashMap::new();
        let mut i18n_generic_names = HashMap::new();
        let mut i18n_comments = HashMap::new();
        let mut i18n_keywords = HashMap::new();

        for (sec, prop) in destktop_ini.iter() {
            if sec != Some("Desktop Entry") {
                continue;
            }
            for (k, v) in prop.iter() {
                if populate_i18n("Name[", k.to_string(), v.to_string(), &mut i18n_names) {
                    continue;
                }
                if populate_i18n(
                    "GenericName[",
                    k.to_string(),
                    v.to_string(),
                    &mut i18n_generic_names,
                ) {
                    continue;
                }
                if populate_i18n("Comment[", k.to_string(), v.to_string(), &mut i18n_comments) {
                    continue;
                }

                if k.starts_with("Keywords[") {
                    if let Some(lang) = k.strip_prefix("Keywords[").unwrap().strip_suffix(']') {
                        i18n_keywords.insert(
                            lang.to_string(),
                            v.split(';')
                                .filter(|k| !k.is_empty())
                                .map(String::from)
                                .collect(),
                        );
                    }
                    continue;
                }
            }
        }
        let section = Some("Desktop Entry");
        DesktopFile {
            default_name: destktop_ini.get_from(section, "Name").map(str::to_string),
            default_generic_name: destktop_ini
                .get_from(section, "GenericName")
                .map(str::to_string),
            default_comment: destktop_ini
                .get_from(section, "Comment")
                .map(str::to_string),

            default_keywords: match destktop_ini.get_from(section, "Keywords") {
                Some(keywords) => keywords
                    .split(';')
                    .filter(|k| !k.is_empty())
                    .map(String::from)
                    .collect(),
                None => vec![],
            },

            mime_type: match destktop_ini.get_from(section, "MimeType") {
                Some(mime_t) => mime_t
                    .split(';')
                    .filter(|m| !m.is_empty())
                    .map(String::from)
                    .collect(),
                None => vec![],
            },

            i18n_names,
            i18n_generic_names,
            i18n_comments,
            i18n_keywords,
        }
    }
}
