-- Contains every character that ever tried to login, if the login was not
-- successful and the user tried again, the user will be here multiple times
CREATE TABLE IF NOT EXISTS credentials(
    created                 TIMESTAMPTZ DEFAULT NOW(),

    character_id            INTEGER,
    character_main          INTEGER,

    -- intention of the user
    -- e.g. LOGIN, LOGIN_ALT, REAUTH
    intention               VARCHAR NOT NULL,
    credential_type         VARCHAR NOT NULL,

    -- token so that we can verify the user
    token                   UUID    NOT NULL DEFAULT gen_random_uuid(),
    -- EVE tokens
    refresh_token           VARCHAR,
    access_token            VARCHAR,

    PRIMARY KEY(token)
);

-- Contains all characters that successfully logged in
CREATE TABLE IF NOT EXISTS characters(
    id                      UUID        NOT NULL DEFAULT gen_random_uuid(),

    character_id            INTEGER     NOT NULL,
    corporation_id          INTEGER     NOT NULL,

    character_name          VARCHAR(50) NOT NULL,
    corporation_name        VARCHAR(50) NOT NULL,

    alliance_id             INTEGER,
    alliance_name           VARCHAR(50),

    PRIMARY KEY (character_id),

    FOREIGN KEY (character_main)
        REFERENCES characters (character_id)
        ON DELETE CASCADE
);
CREATE UNIQUE INDEX IF NOT EXISTS character_id ON characters(id);

-- default user, used for API calls that do not need an authenticated user
INSERT INTO characters (character_id, corporation_id, character_name, corporation_name)
VALUES (0, 0, 'Default', 'Default')
ON CONFLICT DO NOTHING;
