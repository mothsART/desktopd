CREATE TABLE comments (
    id INTEGER PRIMARY KEY NOT NULL,
    app_id INTEGER NOT NULL,
    title VARCHAR NOT NULL,
    lang VARCHAR NULL,

    UNIQUE(id)
    FOREIGN KEY (app_id) REFERENCES app (id)
)
