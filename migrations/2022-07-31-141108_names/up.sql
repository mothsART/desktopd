CREATE TABLE names (
    id INTEGER PRIMARY KEY,
    app_id INTEGER,
    title VARCHAR NOT NULL,
    lang VARCHAR NOT NULL,

    UNIQUE(id)
)
