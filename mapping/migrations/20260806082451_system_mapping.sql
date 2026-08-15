CREATE TABLE IF NOT EXISTS structure(
    id              UUID    NOT NULL DEFAULT uuidv7(),
    owner           INTEGER NOT NULL,

    structure_id    BIGINT  NOT NULL,
    structure_owner INTEGER NOT NULL,
    system_id       INTEGER NOT NULL,
    type_id         INTEGER NOT NULL,

    name            VARCHAR NOT NULL,
    x               REAL    NOT NULL,
    y               REAL    NOT NULL,
    z               REAL    NOT NULL,

    PRIMARY KEY(id)
);
CREATE UNIQUE INDEX IF NOT EXISTS structure_structureid ON structure(structure_id);

CREATE TABLE IF NOT EXISTS queue_system(
    id              UUID    NOT NULL DEFAULT uuidv7(),
    status          VARCHAR NOT NULL DEFAULT 'WAITING',

    system_id       INTEGER NOT NULL,
    system_name     VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS queue_structure(
    id              UUID    NOT NULL DEFAULT uuidv7(),
    status          VARCHAR NOT NULL DEFAULT 'WAITING',

    system_id       INTEGER NOT NULL,
    structure_id    BIGINT  NOT NULL
);
