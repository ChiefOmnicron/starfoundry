-- fix the updated_at_trigger for structure
CREATE OR REPLACE TRIGGER set_updated_at
    AFTER INSERT OR UPDATE ON structure
    EXECUTE FUNCTION trigger_set_updated_at();

CREATE TABLE IF NOT EXISTS industry_hub (
    id            UUID         NOT NULL DEFAULT uuidv7(),
    owner         INTEGER      NOT NULL,

    name          VARCHAR(100) NOT NULL,

    created_at    TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ  NOT NULL DEFAULT NOW(),

    PRIMARY KEY (id)
);
CREATE OR REPLACE TRIGGER set_updated_at
    AFTER INSERT OR UPDATE ON industry_hub
    EXECUTE FUNCTION trigger_set_updated_at();

CREATE TABLE IF NOT EXISTS industry_hub_structure (
    industry_hub_id UUID NOT NULL,
    structure_id       UUID NOT NULL,

    FOREIGN KEY (structure_id)
        REFERENCES structure (id)
        ON DELETE NO ACTION
);
CREATE INDEX IF NOT EXISTS industry_hub_id ON industry_hub_structure (industry_hub_id);
