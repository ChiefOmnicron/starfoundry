-- Add migration script here
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

CREATE OR REPLACE FUNCTION expire_refresh_token()
RETURNS TRIGGER AS $$
BEGIN
  DELETE FROM jwt_refresh_token WHERE created_at < NOW() - INTERVAL '1 day';
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TABLE IF NOT EXISTS login_attempt (
    token               UUID        NOT NULL DEFAULT uuidv7(),
    domain              VARCHAR     NOT NULL,
    credential_type     VARCHAR     NOT NULL,

    -- must be set when logging in a corporation or an alt character
    eve_character_id    INTEGER,

    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    PRIMARY KEY(token)
);
CREATE OR REPLACE TRIGGER expire_login_trigger
    AFTER INSERT ON login_attempt
    EXECUTE FUNCTION expire_login();

CREATE TABLE IF NOT EXISTS jwt_refresh_token (
    id               UUID        NOT NULL DEFAULT uuidv7(),

    eve_character_id INTEGER     NOT NULL,
    refresh_token    VARCHAR     NOT NULL,

    token_hash       VARCHAR(64) NOT NULL,

    created_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    PRIMARY KEY(token_hash)
);
CREATE OR REPLACE TRIGGER expire_refresh_token_trigger
    AFTER INSERT ON jwt_refresh_token
    EXECUTE FUNCTION expire_refresh_token();

CREATE TABLE IF NOT EXISTS provider_eve_credential (
    character_id    INTEGER     NOT NULL,
    refresh_token   VARCHAR     NOT NULL,

    -- if the credential is for a corporation or an alt character, the actual
    -- main character_id will be in here
    character_main  INTEGER,

    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    PRIMARY KEY(character_id)
);
CREATE INDEX IF NOT EXISTS provider_eve_credential_character_main ON provider_eve_credential (character_main);
CREATE OR REPLACE TRIGGER set_updated_at
    AFTER INSERT ON provider_eve_credential
    EXECUTE FUNCTION trigger_set_updated_at();
