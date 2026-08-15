CREATE UNLOGGED TABLE IF NOT EXISTS system_cache (
    region_id           INTEGER NOT NULL,
    constellation_id    INTEGER NOT NULL,
    system_id           INTEGER NOT NULL,

    region_name         VARCHAR NOT NULL,
    constellation_name  VARCHAR NOT NULL,
    system_name         VARCHAR NOT NULL,

    security            REAL    NOT NULL,
    security_str        VARCHAR NOT NULl,

    PRIMARY KEY(system_id)
);

CREATE UNLOGGED TABLE IF NOT EXISTS system_distance_cache(
    system_start    INTEGER NOT NULL,
    system_end      INTEGER NOT NULL,
    distance_ly     REAL    NOT NULL,

    PRIMARY KEY(system_start, system_end)
);
