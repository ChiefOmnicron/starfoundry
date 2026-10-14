CREATE TABLE IF NOT EXISTS filter(
    id              UUID        NOT NULL DEFAULT uuidv7(),

    name            VARCHAR     NOT NULL,
    typ             VARCHAR     NOT NULL,
    filter          JSONB       NOT NULL,

    owner           INTEGER     NOT NULL,

    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE OR REPLACE TRIGGER set_updated_at
    AFTER INSERT ON filter
    EXECUTE FUNCTION trigger_set_updated_at();
