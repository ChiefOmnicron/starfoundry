CREATE TABLE IF NOT EXISTS global_filter(
    id              UUID        NOT NULL DEFAULT uuidv7(),

    typ             VARCHAR     NOT NULL,
    filter          JSONB       NOT NULL,

    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE OR REPLACE TRIGGER set_updated_at
    AFTER INSERT ON global_filter
    EXECUTE FUNCTION trigger_set_updated_at();
