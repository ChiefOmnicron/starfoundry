CREATE TABLE IF NOT EXISTS system (
    region_id          INTEGER NOT NULL,
    constellation_id   INTEGER NOT NULL,
    system_id          INTEGER NOT NULL,

    region_name        VARCHAR NOT NULL,
    constellation_name VARCHAR NOT NULL,
    system_name        VARCHAR NOT NULL,

    security           REAL    NOT NULL,
    security_str       VARCHAR NOT NULL,

    PRIMARY KEY(system_id)
);
CREATE INDEX IF NOT EXISTS systems_id ON system (region_id, constellation_id, system_id);
