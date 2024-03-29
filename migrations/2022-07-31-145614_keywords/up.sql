CREATE TABLE keywords (
    id INTEGER PRIMARY KEY NOT NULL,
    app_id INTEGER NOT NULL,
    locale_id INTEGER NOT NULL,
    key VARCHAR NOT NULL,

    UNIQUE(id)
    UNIQUE(app_id, key)

    FOREIGN KEY (app_id) REFERENCES app (id)
    FOREIGN KEY (locale_id) REFERENCES locale (id)
)
