-- updates the updated_at field automatically
CREATE OR REPLACE FUNCTION trigger_set_updated_at()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TABLE IF NOT EXISTS character (
    character_id            INTEGER     NOT NULL,
    character_name          VARCHAR(50) NOT NULL,

    corporation_id          INTEGER     NOT NULL,
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
