CREATE TABLE keywords (
    id INTEGER PRIMARY KEY,
    app_id INTEGER,
    key VARCHAR NOT NULL,
    lang VARCHAR NULL,

    UNIQUE(id)
)
