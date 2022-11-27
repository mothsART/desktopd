CREATE TABLE locale (
    id INTEGER PRIMARY KEY NOT NULL,
    key VARCHAR NOT NULL,
    
    UNIQUE(id)
    UNIQUE (key)
)
