DO
$$
BEGIN
    IF NOT EXISTS (
        SELECT *
        FROM pg_type typ
        INNER JOIN pg_namespace nsp ON nsp.oid = typ.typnamespace
        WHERE nsp.nspname = current_schema()
        AND typ.typname = 'share_type'
    ) THEN
        CREATE TYPE SHARE_TYPE AS ENUM (
            'CHARACTER',
            'CORPORATION',
            'ALLIANCE'
        );
    END IF;
END;
$$
LANGUAGE plpgsql;

CREATE TABLE IF NOT EXISTS industry_hub_share (
    id              UUID        NOT NULL DEFAULT uuidv7(),

    -- id of the industry hub the share is assigned to
    industry_hub_id UUID        NOT NULL,

    share_id        INTEGER     NOT NULL,
    share_type      SHARE_TYPE  NOT NULL,

    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    FOREIGN KEY (industry_hub_id)
        REFERENCES industry_hub (id)
        ON DELETE CASCADE
);
CREATE OR REPLACE TRIGGER set_updated_at
    AFTER INSERT ON industry_hub_share
    EXECUTE FUNCTION trigger_set_updated_at();

CREATE TABLE IF NOT EXISTS industry_hub_clone (
    id              UUID NOT NULL DEFAULT uuidv7(),

    -- id of the original industry hub
    -- might be null if the original owner deleted it
    original_id     UUID,

    -- id of the clone
    new_id          UUID NOT NULL,

    created_at      TIMESTAMPTZ     NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ     NOT NULL DEFAULT NOW(),

    -- set the original null of the original owner deleted it
    FOREIGN KEY (original_id)
        REFERENCES industry_hub (id)
        ON DELETE SET NULL,

    -- delete the clone if the new industry hub is deleted
    FOREIGN KEY (new_id)
        REFERENCES industry_hub (id)
        ON DELETE CASCADE
);
CREATE OR REPLACE TRIGGER set_updated_at
    AFTER INSERT ON industry_hub_clone
    EXECUTE FUNCTION trigger_set_updated_at();

CREATE TABLE IF NOT EXISTS structure_share (
    id              UUID        NOT NULL DEFAULT uuidv7(),

    -- id of the industry hub the share is assigned to
    structure_id    UUID        NOT NULL,

    share_id        INTEGER     NOT NULL,
    share_type      SHARE_TYPE  NOT NULL,

    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    FOREIGN KEY (structure_id)
        REFERENCES structure (id)
        ON DELETE CASCADE
);
CREATE OR REPLACE TRIGGER set_updated_at
    AFTER INSERT ON structure_share
    EXECUTE FUNCTION trigger_set_updated_at();

CREATE TABLE IF NOT EXISTS structure_clone (
    id              UUID NOT NULL DEFAULT uuidv7(),

    -- id of the original structure
    -- might be null if the original owner deleted it
    original_id     UUID,

    -- id of the clone
    new_id          UUID NOT NULL,

    created_at      TIMESTAMPTZ     NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ     NOT NULL DEFAULT NOW(),

    -- set the original null of the original owner deleted it
    FOREIGN KEY (original_id)
        REFERENCES structure (id)
        ON DELETE SET NULL,

    -- delete the clone if the new structure is deleted
    FOREIGN KEY (new_id)
        REFERENCES structure (id)
        ON DELETE CASCADE
);
CREATE OR REPLACE TRIGGER set_updated_at
    AFTER INSERT ON structure_clone
    EXECUTE FUNCTION trigger_set_updated_at();
