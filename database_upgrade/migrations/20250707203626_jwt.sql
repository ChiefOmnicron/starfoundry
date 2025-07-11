CREATE TABLE jwt_refresh_token(
    character_id      INTEGER NOT NULL,
    refresh_token     VARCHAR NOT NULL,

    created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    PRIMARY KEY (character_id, refresh_token)
);
