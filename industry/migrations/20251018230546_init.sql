-- updates the updated_at field automatically
CREATE OR REPLACE FUNCTION trigger_set_updated_at()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TABLE IF NOT EXISTS structure (
    id           UUID            NOT NULL DEFAULT uuidv7(),
    owner        INTEGER         NOT NULL,
    structure_id BIGINT          NOT NULL,

    system_id    INTEGER         NOT NULL,

    -- Structure Type id
    type_id      INTEGER         NOT NULL,
    rigs         INTEGER[]       NOT NULL DEFAULT ARRAY[]::INTEGER[],
    services     INTEGER[]       NOT NULL DEFAULT ARRAY[]::INTEGER[],

    name         VARCHAR(128)    NOT NULL,

    created_at   TIMESTAMPTZ     NOT NULL DEFAULT NOW(),
    updated_at   TIMESTAMPTZ     NOT NULL DEFAULT NOW(),

    PRIMARY KEY (id, owner)
);
CREATE UNIQUE INDEX IF NOT EXISTS structure_id ON structure (id);
CREATE OR REPLACE TRIGGER set_updated_at
    AFTER INSERT ON structure
    EXECUTE FUNCTION trigger_set_updated_at();
