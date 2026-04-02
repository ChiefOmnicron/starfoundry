CREATE TABLE IF NOT EXISTS solution(
    id                  UUID        NOT NULL DEFAULT uuidv7(),

    industry_hub_id     UUID        NOT NULL,
    project_group_id    UUID        NOT NULL,

    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    PRIMARY KEY(id)
);
CREATE OR REPLACE TRIGGER set_updated_at
    AFTER INSERT ON solution
    EXECUTE FUNCTION trigger_set_updated_at();

CREATE TABLE IF NOT EXISTS solution_blacklist(
    id          UUID        NOT NULL DEFAULT uuidv7(),
    solution_id UUID        NOT NULL,

    type_id     INTEGER     NOT NULL,

    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    PRIMARY KEY(id),
    FOREIGN KEY (solution_id)
        REFERENCES solution(id)
        ON DELETE CASCADE
);
CREATE OR REPLACE TRIGGER set_updated_at
    AFTER INSERT ON solution_blacklist
    EXECUTE FUNCTION trigger_set_updated_at();

CREATE TABLE IF NOT EXISTS solution_blueprint_overwrite(
    id                  UUID        NOT NULL DEFAULT uuidv7(),
    solution_id         UUID        NOT NULL,

    type_id             INTEGER     NOT NULL,
    material_efficiency INTEGER     NOT NULL,

    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    PRIMARY KEY(id),
    FOREIGN KEY (solution_id)
        REFERENCES solution(id)
        ON DELETE CASCADE
);
CREATE OR REPLACE TRIGGER set_updated_at
    AFTER INSERT ON solution_blueprint_overwrite
    EXECUTE FUNCTION trigger_set_updated_at();

CREATE TABLE IF NOT EXISTS solution_job_split(
    id          UUID        NOT NULL DEFAULT uuidv7(),
    solution_id         UUID        NOT NULL,

    type_id     INTEGER     NOT NULL,
    runs        INTEGER     NOT NULL,

    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    PRIMARY KEY(id),
    FOREIGN KEY (solution_id)
        REFERENCES solution(id)
        ON DELETE CASCADE
);
CREATE OR REPLACE TRIGGER set_updated_at
    AFTER INSERT ON solution_job_split
    EXECUTE FUNCTION trigger_set_updated_at();

CREATE TABLE IF NOT EXISTS solution_stock(
    id          UUID        NOT NULL DEFAULT uuidv7(),
    solution_id         UUID        NOT NULL,

    type_id     INTEGER     NOT NULL,
    quantity    INTEGER     NOT NULL,

    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    PRIMARY KEY(id),
    FOREIGN KEY (solution_id)
        REFERENCES solution(id)
        ON DELETE CASCADE
);
CREATE OR REPLACE TRIGGER set_updated_at
    AFTER INSERT ON solution_stock
    EXECUTE FUNCTION trigger_set_updated_at();

CREATE TABLE IF NOT EXISTS solution_product(
    id                  UUID        NOT NULL DEFAULT uuidv7(),
    solution_id         UUID        NOT NULL,

    type_id             INTEGER     NOT NULL,
    quantity            INTEGER     NOT NULL,
    material_efficiency INTEGER     NOT NULL DEFAULT 10,

    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    PRIMARY KEY(id),
    FOREIGN KEY (solution_id)
        REFERENCES solution(id)
        ON DELETE CASCADE
);
CREATE OR REPLACE TRIGGER set_updated_at
    AFTER INSERT ON solution_product
    EXECUTE FUNCTION trigger_set_updated_at();

CREATE TABLE IF NOT EXISTS solution_excess(
    id                  UUID        NOT NULL DEFAULT uuidv7(),
    solution_id         UUID        NOT NULL,

    type_id             INTEGER     NOT NULL,
    quantity            INTEGER     NOT NULL,

    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    PRIMARY KEY(id),
    FOREIGN KEY (solution_id)
        REFERENCES solution(id)
        ON DELETE CASCADE
);
CREATE OR REPLACE TRIGGER set_updated_at
    AFTER INSERT ON solution_excess
    EXECUTE FUNCTION trigger_set_updated_at();

CREATE TABLE IF NOT EXISTS solution_material(
    id                  UUID        NOT NULL DEFAULT uuidv7(),
    solution_id         UUID        NOT NULL,

    type_id             INTEGER     NOT NULL,
    quantity            INTEGER     NOT NULL,

    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    PRIMARY KEY(id),
    FOREIGN KEY (solution_id)
        REFERENCES solution(id)
        ON DELETE CASCADE
);
CREATE OR REPLACE TRIGGER set_updated_at
    AFTER INSERT ON solution_material
    EXECUTE FUNCTION trigger_set_updated_at();

CREATE TABLE IF NOT EXISTS solution_manufacturing(
    id                  UUID        NOT NULL DEFAULT uuidv7(),
    solution_id         UUID        NOT NULL,

    type_id             INTEGER     NOT NULL,
    runs                INTEGER     NOT NULL,
    structure_id        UUID,

    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    PRIMARY KEY(id),
    FOREIGN KEY (solution_id)
        REFERENCES solution(id)
        ON DELETE CASCADE
);
CREATE OR REPLACE TRIGGER set_updated_at
    AFTER INSERT ON solution_material
    EXECUTE FUNCTION trigger_set_updated_at();

ALTER TABLE project DROP COLUMN IF EXISTS industry_hub_id;
ALTER TABLE project ADD COLUMN IF NOT EXISTS solution_id UUID;
ALTER TABLE project DROP CONSTRAINT IF EXISTS project_solution_id;
ALTER TABLE project ADD CONSTRAINT project_solution_id FOREIGN KEY (solution_id) REFERENCES solution(id) ON DELETE SET NULL;

--DROP TABLE project_product;
--DROP TABLE project_blacklist;
