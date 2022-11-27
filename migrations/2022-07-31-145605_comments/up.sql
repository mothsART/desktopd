CREATE TABLE comments (
    id INTEGER PRIMARY KEY NOT NULL,
    app_id INTEGER NOT NULL,
    locale_id INTEGER NOT NULL,
    title VARCHAR NOT NULL,

    UNIQUE(id)
    UNIQUE(app_id, title)

    FOREIGN KEY (app_id) REFERENCES app (id)
    FOREIGN KEY (locale_id) REFERENCES locale (id)
)
