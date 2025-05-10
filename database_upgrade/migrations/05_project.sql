-- represents all different variations of an industry job
DO
$$
BEGIN
    IF NOT EXISTS (
        SELECT *
        FROM pg_type typ
        INNER JOIN pg_namespace nsp ON nsp.oid = typ.typnamespace
        WHERE nsp.nspname = current_schema()
        AND typ.typname = 'industry_activity'
    ) THEN
        CREATE TYPE INDUSTRY_ACTIVITY AS ENUM (
            'MANUFACTURING',
            'TIME_EFFICIENCY_RESEARCH',
            'MATERIAL_EFFICIENCY_RESEARCH',
            'COPYING',
            'INVENTION',
            'REACTIONS',
            'UNKNOWN'
        );
    END IF;
END;
$$
LANGUAGE plpgsql;

-- represents all different variations of statuses a project can have
DO
$$
BEGIN
    IF NOT EXISTS (
        SELECT *
        FROM pg_type typ
        INNER JOIN pg_namespace nsp ON nsp.oid = typ.typnamespace
        WHERE nsp.nspname = current_schema()
        AND typ.typname = 'project_status'
    ) THEN
        CREATE TYPE PROJECT_STATUS AS ENUM (
            'PREPARING',
            'IN_PROGRESS',
            'PAUSED',
            'DONE'
        );
    END IF;
END;
$$
LANGUAGE plpgsql;

-- represents the different statuses a job can have
DO
$$
BEGIN
    IF NOT EXISTS (
        SELECT *
        FROM pg_type typ
        INNER JOIN pg_namespace nsp ON nsp.oid = typ.typnamespace
        WHERE nsp.nspname = current_schema()
        AND typ.typname = 'project_job_status'
    ) THEN
        CREATE TYPE PROJECT_JOB_STATUS AS ENUM (
            'WAITING_FOR_MATERIALS',
            'BUILDING',
            'DONE'
        );
    END IF;
END;
$$
LANGUAGE plpgsql;

-- list of all projects that were created
CREATE TABLE IF NOT EXISTS project (
    id                 UUID             NOT NULL DEFAULT gen_random_uuid(),

    sell_price         DOUBLE PRECISION,

    owner              INTEGER          NOT NULL,

    structure_group_id UUID             NOT NULL,
    project_group_id   UUID             NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000',

    status             PROJECT_STATUS   NOT NULL DEFAULT 'PREPARING',

    orderer            VARCHAR(128)     NOT NULL,
    name               VARCHAR(128)     NOT NULL,
    note               VARCHAR(2048),

    created_at         TIMESTAMPTZ      NOT NULL DEFAULT NOW(),
    updated_at         TIMESTAMPTZ      NOT NULL DEFAULT NOW(),

    PRIMARY KEY (id),

    FOREIGN KEY (project_group_id)
        REFERENCES project_group (id)
        ON DELETE SET DEFAULT,

    FOREIGN KEY (structure_group_id)
        REFERENCES structure_group (id)
        ON DELETE NO ACTION
);

-- list of items that should be produced in a project
CREATE TABLE IF NOT EXISTS project_product (
    id                  UUID        NOT NULL DEFAULT gen_random_uuid(),
    project_id          UUID        NOT NULL,

    type_id             INTEGER     NOT NULL,
    quantity            INTEGER     NOT NULL,
    material_efficiency INTEGER     NOT NULL DEFAULT 0,

    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    FOREIGN KEY (project_id)
        REFERENCES project (id)
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS project_job (
    id           UUID               NOT NULL DEFAULT gen_random_uuid(),
    project_id   UUID               NOT NULL,

    type_id      INTEGER            NOT NULL,
    runs         INTEGER            NOT NULL,

    structure_id UUID               NOT NULL,
    -- EVE Character id of the character that started the job
    character_id INTEGER,

    status       PROJECT_JOB_STATUS NOT NULL DEFAULT 'WAITING_FOR_MATERIALS',
    cost         DOUBLE PRECISION,
    -- ID of the job given by CCP
    job_id       INTEGER,

    created_at   TIMESTAMPTZ        NOT NULL DEFAULT NOW(),
    updated_at   TIMESTAMPTZ        NOT NULL DEFAULT NOW(),

    FOREIGN KEY (project_id)
        REFERENCES project (id)
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS project_market (
    id           UUID             NOT NULL DEFAULT gen_random_uuid(),
    project_id   UUID             NOT NULL,

    type_id      INTEGER          NOT NULL,
    quantity     INTEGER          NOT NULL,

    cost         DOUBLE PRECISION,

    source       VARCHAR(128)     NOT NULL DEFAULT 'Unknown',

    created_at   TIMESTAMPTZ      NOT NULL DEFAULT NOW(),
    updated_at   TIMESTAMPTZ      NOT NULL DEFAULT NOW(),

    FOREIGN KEY (project_id)
        REFERENCES project (id)
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS project_misc (
    id           UUID             NOT NULL DEFAULT gen_random_uuid(),
    project_id   UUID             NOT NULL,

    item         VARCHAR          NOT NULL,

    cost         DOUBLE PRECISION NOT NULL,

    quantity     INTEGER,
    description  VARCHAR(128),

    created_at   TIMESTAMPTZ      NOT NULL DEFAULT NOW(),
    updated_at   TIMESTAMPTZ      NOT NULL DEFAULT NOW(),

    FOREIGN KEY (project_id)
        REFERENCES project (id)
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS project_excess (
    project_id   UUID             NOT NULL,

    type_id      INTEGER          NOT NULL,
    quantity     INTEGER          NOT NULL,

    cost         DOUBLE PRECISION,

    created_at   TIMESTAMPTZ      NOT NULL DEFAULT NOW(),
    updated_at   TIMESTAMPTZ      NOT NULL DEFAULT NOW(),

    FOREIGN KEY (project_id)
        REFERENCES project (id)
        ON DELETE CASCADE
);
CREATE UNIQUE INDEX IF NOT EXISTS project_excess_type ON project_excess(project_id, type_id);

CREATE TABLE IF NOT EXISTS project_stock (
    project_id   UUID             NOT NULL,

    type_id      INTEGER          NOT NULL,
    quantity     INTEGER          NOT NULL,

    cost         DOUBLE PRECISION,

    created_at   TIMESTAMPTZ      NOT NULL DEFAULT NOW(),
    updated_at   TIMESTAMPTZ      NOT NULL DEFAULT NOW(),

    FOREIGN KEY (project_id)
        REFERENCES project (id)
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS project_market_structure (
    project_id   UUID        NOT NULL,

    structure_id UUID        NOT NULL,

    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    FOREIGN KEY (project_id)
        REFERENCES project (id)
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS project_blacklist (
    project_id   UUID        NOT NULL,

    type_id      INTEGER     NOT NULL,

    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    FOREIGN KEY (project_id)
        REFERENCES project (id)
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS project_job_assignment (
    id      UUID    NOT NULL DEFAULT gen_random_uuid(),

    job_id  UUID    NOT NULL,
    started BOOLEAN NOT NULL DEFAULT FALSE,

    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE UNIQUE INDEX IF NOT EXISTS project_job_assignment_id_job ON project_job_assignment(id, job_id);
