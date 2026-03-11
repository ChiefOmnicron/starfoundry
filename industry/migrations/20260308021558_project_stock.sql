CREATE TABLE IF NOT EXISTS project_stock (
    id          UUID                NOT NULL DEFAULT uuidv7(),
    project_id  UUID                NOT NULL,

    type_id     INTEGER             NOT NULL,
    quantity    INTEGER             NOT NULL,

    cost        DOUBLE PRECISION,

    created_at  TIMESTAMPTZ         NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ         NOT NULL DEFAULT NOW(),

    FOREIGN KEY (project_id)
        REFERENCES project (id)
        ON DELETE CASCADE
);
CREATE OR REPLACE TRIGGER set_updated_at
    AFTER INSERT ON project_stock
    EXECUTE FUNCTION trigger_set_updated_at();
