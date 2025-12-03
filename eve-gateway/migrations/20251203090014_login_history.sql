CREATE TABLE login_history (
    id               UUID        NOT NULL DEFAULT uuidv7(),

    character_id     INTEGER     NOT NULL,
    source           VARCHAR     NOT NULL,

    created_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY(id)
)
