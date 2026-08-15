CREATE TABLE IF NOT EXISTS standing(
    id              UUID    NOT NULL DEFAULT uuidv7(),
    owner_id        INTEGER NOT NULL,

    -- either an alliance, corporation or character
    contact_id      INTEGER NOT NULL,
    contact_type    VARCHAR NOT NULL,

    standing        REAL    NOT NULL,

    -- either an alliance, corporation or character
    -- determines the source of the standing
    source          VARCHAR NOT NULL,

    PRIMARY KEY(id)
);
CREATE INDEX IF NOT EXISTS standing_ownerid ON standing (owner_id);
