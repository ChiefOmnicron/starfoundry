-- updates the updated_at field automatically
CREATE OR REPLACE FUNCTION trigger_set_updated_at()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION expire_login()
RETURNS TRIGGER AS $$
BEGIN
  DELETE FROM login_attempt WHERE created_at < NOW() - INTERVAL '20 minute';
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TABLE IF NOT EXISTS character(
    uuid                    UUID        NOT NULL DEFAULT gen_random_uuid(),

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
CREATE UNIQUE INDEX IF NOT EXISTS character_id ON character(character_id);
CREATE OR REPLACE TRIGGER set_updated_at
    AFTER INSERT ON character
    EXECUTE FUNCTION trigger_set_updated_at();

CREATE TABLE IF NOT EXISTS login_attempt(
    token               UUID        NOT NULL DEFAULT gen_random_uuid(),
    credential_type     VARCHAR     NOT NULL,

    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    PRIMARY KEY(token)
);
CREATE OR REPLACE TRIGGER expire_login_trigger
    AFTER INSERT ON login_attempt
    EXECUTE FUNCTION expire_login();
