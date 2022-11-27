CREATE TABLE app_locale (
    id INTEGER PRIMARY KEY NOT NULL,
    app_id INTEGER NOT NULL,
    locale_id INTEGER NOT NULL,

    UNIQUE(id)
    FOREIGN KEY (app_id) REFERENCES app (id)
    FOREIGN KEY (locale_id) REFERENCES locale (id)
)
