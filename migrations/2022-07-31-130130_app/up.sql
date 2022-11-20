CREATE TABLE app (
    id INTEGER PRIMARY KEY NOT NULL,
    title VARCHAR NOT NULL,
    path VARCHAR NOT NULL,
    generic_title VARCHAR NULL,
    comment VARCHAR NULL,
    exec VARCHAR NULL,
    try_exec VARCHAR NULL,
    icon_path VARCHAR NULL,

    UNIQUE(id, path)
)
