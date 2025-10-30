CREATE TABLE IF NOT EXISTS category(
    category_id INTEGER NOT NULL,
    name        VARCHAR NOT NULL,

    PRIMARY KEY(category_id)
);

CREATE TABLE IF NOT EXISTS groups(
    group_id    INTEGER NOT NULL,
    category_id INTEGER NOT NULL,
    name        VARCHAR NOT NULL,

    PRIMARY KEY(group_id)
);
