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

CREATE TABLE IF NOT EXISTS projects(
    id                 UUID             NOT NULL DEFAULT gen_random_uuid(),

    owner              INTEGER          NOT NULL,
    name               VARCHAR          NOT NULL,

    structure_group_id UUID             NOT NULL,
    project_group_id   UUID             NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000',

    status             PROJECT_STATUS   NOT NULL DEFAULT 'PREPARING',

    created_at         TIMESTAMPTZ      NOT NULL DEFAULT NOW(),

    sell_price         DOUBLE PRECISION,

    orderer            VARCHAR          NOT NULL,
    notes              VARCHAR,

    PRIMARY KEY (id),
    FOREIGN KEY (group_id)
        REFERENCES project_groups (id)
        ON DELETE SET DEFAULT
);

-- List of items that should be produced in a project
CREATE TABLE IF NOT EXISTS project_products(
    id                  UUID    NOT NULL DEFAULT gen_random_uuid(),
    project_id          UUID    NOT NULL,

    type_id             INTEGER NOT NULL,
    quantity            INTEGER NOT NULL,
    material_efficiency INTEGER NOT NULL DEFAULT 0,

    FOREIGN KEY (project_id)
        REFERENCES projects (id)
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS project_jobs(
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

    FOREIGN KEY (project_id)
        REFERENCES projects (id)
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS project_market(
    id           UUID    NOT NULL DEFAULT gen_random_uuid(),
    project_id   UUID    NOT NULL,

    type_id      INTEGER NOT NULL,
    quantity     INTEGER NOT NULL,

    cost         DOUBLE PRECISION,

    source       VARCHAR,

    FOREIGN KEY (project_id)
        REFERENCES projects (id)
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS project_misc(
    id           UUID             NOT NULL DEFAULT gen_random_uuid(),
    project_id   UUID             NOT NULL,

    item         VARCHAR          NOT NULL,

    cost         DOUBLE PRECISION NOT NULL,

    quantity     INTEGER,
    description  VARCHAR,

    FOREIGN KEY (project_id)
        REFERENCES projects (id)
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS project_excess(
    project_id   UUID             NOT NULL,

    type_id      INTEGER          NOT NULL,
    quantity     INTEGER          NOT NULL,

    cost         DOUBLE PRECISION,

    FOREIGN KEY (project_id)
        REFERENCES projects (id)
        ON DELETE CASCADE
);
CREATE UNIQUE INDEX IF NOT EXISTS project_excess_type ON project_excess(project_id, type_id);

CREATE TABLE IF NOT EXISTS project_stocks(
    project_id   UUID             NOT NULL,

    type_id      INTEGER          NOT NULL,
    quantity     INTEGER          NOT NULL,

    cost         DOUBLE PRECISION,

    FOREIGN KEY (project_id)
        REFERENCES projects (id)
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS project_market_structures(
    project_id   UUID NOT NULL,

    structure_id UUID NOT NULL,

    FOREIGN KEY (project_id)
        REFERENCES projects (id)
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS project_blacklist(
    project_id   UUID    NOT NULL,

    type_id      INTEGER NOT NULL,

    FOREIGN KEY (project_id)
        REFERENCES projects (id)
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS project_job_assignments (
    id      UUID    NOT NULL DEFAULT gen_random_uuid(),

    job_id  UUID    NOT NULL,
    started BOOLEAN NOT NULL DEFAULT FALSE
);
CREATE UNIQUE INDEX IF NOT EXISTS project_job_assignments_id_job ON project_job_assignments(id, job_id);

CREATE TABLE IF NOT EXISTS project_groups(
    id          UUID    NOT NULL DEFAULT gen_random_uuid(),

    owner       INTEGER NOT NULL,

    name        VARCHAR NOT NULL,
    description VARCHAR,

    PRIMARY KEY (id),
    FOREIGN KEY (owner)
        REFERENCES characters(character_id)
        ON DELETE CASCADE
);

INSERT INTO project_groups (id, owner, name, description)
VALUES ('00000000-0000-0000-0000-000000000000', 0, 'Default', 'Default Group')
ON CONFLICT DO NOTHING;

CREATE TABLE IF NOT EXISTS project_group_members(
    id                UUID    NOT NULL DEFAULT gen_random_uuid(),
    group_id          UUID    NOT NULL,

    -- character id
    character_id      INTEGER NOT NULL,

    accepted          BOOLEAN NOT NULL DEFAULT FALSE,

    -- permissions for projects
    projects          VARCHAR NOT NULL DEFAULT 'READ',
    -- permissions for projects
    project_group     VARCHAR NOT NULL DEFAULT 'READ',
    -- permissions for structures
    structures        VARCHAR NOT NULL DEFAULT 'READ',

    PRIMARY KEY (group_id, id),
    FOREIGN KEY (group_id)
        REFERENCES project_groups(id)
        ON DELETE CASCADE
);
CREATE UNIQUE INDEX IF NOT EXISTS project_group_members_group_character ON project_group_members(group_id, character_id);

INSERT INTO project_group_members (group_id, character_id, accepted, projects, structures)
VALUES ('00000000-0000-0000-0000-000000000000', 0, TRUE, 'WRITE', 'WRITE')
ON CONFLICT DO NOTHING;

CREATE TABLE IF NOT EXISTS project_group_default_markets(
    project_group_id UUID NOT NULL,

    structure_id     UUID NOT NULL,

    FOREIGN KEY (project_group_id)
        REFERENCES project_groups (id)
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS project_group_default_blacklist(
    project_group_id UUID    NOT NULL,

    type_id          INTEGER NOT NULL,

    FOREIGN KEY (project_group_id)
        REFERENCES project_groups (id)
        ON DELETE CASCADE
);
