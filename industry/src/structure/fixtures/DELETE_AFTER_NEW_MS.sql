CREATE TABLE IF NOT EXISTS system (
    region_id          INTEGER NOT NULL,
    constellation_id   INTEGER NOT NULL,
    system_id          INTEGER NOT NULL,

    region_name        VARCHAR NOT NULL,
    constellation_name VARCHAR NOT NULL,
    system_name        VARCHAR NOT NULL,

    security           REAL    NOT NULL,

    PRIMARY KEY(system_id)
);
CREATE INDEX IF NOT EXISTS systems_id ON system (region_id, constellation_id, system_id);

CREATE TABLE IF NOT EXISTS item (
    type_id        INTEGER NOT NULL,
    category_id    INTEGER NOT NULL,
    group_id       INTEGER NOT NULL,

    meta_group_id  INTEGER,

    volume         REAL    NOT NULL,

    name           VARCHAR NOT NULL,

    repackaged     INTEGER,

    PRIMARY KEY(type_id)
);
