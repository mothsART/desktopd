CREATE TABLE generic_names (
    id INTEGER PRIMARY KEY,
    app_id INTEGER,
    title VARCHAR NOT NULL,
    lang VARCHAR NULL,

    UNIQUE(id)
)
