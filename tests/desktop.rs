#[cfg(target_os = "linux")]
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::path::Path;

    use desktopd::desktop::DesktopFile;

    #[test]
    fn test_desktop() {
        assert_eq!(
            DesktopFile {
                default_name: "Chromium Web Browser".to_string(),
                default_generic_name: Some("Web Browser".to_string()),
                default_comment: Some("Access the Internet".to_string()),

                default_keywords: vec!(
                    "browser".to_string(),
                    "internet".to_string(),
                    "web".to_string()
                ),

                mime_type: vec!("text/html".to_string(), "text/xml".to_string()),

                i18n_names: HashMap::from([
                    ("de".to_string(), "Chromium-Webbrowser".to_string()),
                    ("es".to_string(), "Navegador web Chromium".to_string()),
                    ("zh_TW".to_string(), "Chromium 網頁瀏覽器".to_string()),
                ]),
                i18n_generic_names: HashMap::from([
                    ("de".to_string(), "Webbrowser".to_string()),
                    ("fr".to_string(), "Navigateur Web".to_string()),
                    ("zh_CN".to_string(), "网页浏览器".to_string()),
                ]),
                i18n_comments: HashMap::from([
                    ("ar".to_string(), "الدخول إلى الإنترنت".to_string()),
                    ("en_GB".to_string(), "Access the Internet".to_string()),
                    ("zh_TW".to_string(), "連線到網際網路".to_string()),
                ]),

                i18n_keywords: HashMap::from([(
                    "fr".to_string(),
                    vec!("navigateur".to_string(), "surfer".to_string())
                ),]),

                exec: None,
                try_exec: None,
                icon: None,
            },
            DesktopFile::new(Path::new("tests/test1/path-1/applications/app-one.desktop"))
        );
    }
}
