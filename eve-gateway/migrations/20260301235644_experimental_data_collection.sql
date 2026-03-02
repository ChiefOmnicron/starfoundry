CREATE UNLOGGED TABLE IF NOT EXISTS eve_cache (
    id               UUID        NOT NULL DEFAULT uuidv7(),
    value            JSONB       NOT NULL,
    hash             VARCHAR     NOT NULL,

    created_at       TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE UNIQUE INDEX hash_cache_key ON eve_cache (hash);
