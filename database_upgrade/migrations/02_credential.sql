-- Contains every character that ever tried to login, if the login was not
-- successful and the user tried again, the user will be here multiple times
CREATE TABLE IF NOT EXISTS credential (
    character_id            INTEGER,
    character_main          INTEGER,

    -- intention of the user
    -- e.g. LOGIN, LOGIN_ALT, REAUTH
    intention               VARCHAR     NOT NULL,
    credential_type         VARCHAR     NOT NULL,

    -- token so that we can verify the user
    token                   UUID        NOT NULL DEFAULT gen_random_uuid(),
    -- EVE tokens
    refresh_token           VARCHAR,
    access_token            VARCHAR,

    created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    PRIMARY KEY(token)
);
CREATE INDEX IF NOT EXISTS credential_character_main ON credential (character_main);

-- contains all characters that successfully logged in
CREATE TABLE IF NOT EXISTS character (
    id                      UUID        NOT NULL DEFAULT gen_random_uuid(),

    character_id            INTEGER     NOT NULL,
    corporation_id          INTEGER     NOT NULL,

    character_name          VARCHAR(50) NOT NULL,
    corporation_name        VARCHAR(50) NOT NULL,

    alliance_id             INTEGER,
    alliance_name           VARCHAR(50),

    created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    PRIMARY KEY (character_id)
);
CREATE UNIQUE INDEX IF NOT EXISTS character_id ON character (id);

-- default user, used for API calls that do not need an authenticated user
--
-- the database migration application will also check if the user exists and if
-- not will insert it
INSERT INTO character (character_id, corporation_id, character_name, corporation_name)
VALUES (0, 0, 'Default', 'Default')
ON CONFLICT DO NOTHING;
