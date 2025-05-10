CREATE TABLE IF NOT EXISTS project_group (
    id          UUID          NOT NULL DEFAULT gen_random_uuid(),

    owner       INTEGER       NOT NULL,

    name        VARCHAR(128)  NOT NULL,
    description VARCHAR(2048),

    created_at   TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at   TIMESTAMPTZ  NOT NULL DEFAULT NOW(),

    PRIMARY KEY (id)
);
-- default project group
--
-- the database migration application is also validating that the group exists,
-- and if not creates it
INSERT INTO project_group (id, owner, name, description)
VALUES ('00000000-0000-0000-0000-000000000000', 0, 'Default', 'Default Group')
ON CONFLICT DO NOTHING;

CREATE TABLE IF NOT EXISTS project_group_member (
    id                UUID        NOT NULL DEFAULT gen_random_uuid(),
    group_id          UUID        NOT NULL,

    -- character id
    character_id      INTEGER     NOT NULL,

    accepted          BOOLEAN     NOT NULL DEFAULT FALSE,

    -- permissions for projects
    projects          VARCHAR     NOT NULL DEFAULT 'READ',
    -- permissions for projects
    project_group     VARCHAR     NOT NULL DEFAULT 'READ',
    -- permissions for structures
    structures        VARCHAR     NOT NULL DEFAULT 'READ',

    created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    PRIMARY KEY (group_id, id),

    FOREIGN KEY (group_id)
        REFERENCES project_group (id)
        ON DELETE CASCADE
);
CREATE UNIQUE INDEX IF NOT EXISTS project_group_member_group_character ON project_group_member(group_id, character_id);
-- inserts a default project member to the default project group
--
-- the database migration application also validates that the user exists and if
-- not inserts it
INSERT INTO project_group_member (group_id, character_id, accepted, projects, structures)
VALUES ('00000000-0000-0000-0000-000000000000', 0, TRUE, 'WRITE', 'WRITE')
ON CONFLICT DO NOTHING;

CREATE TABLE IF NOT EXISTS project_group_default_market (
    project_group_id UUID        NOT NULL,

    structure_id     UUID        NOT NULL,

    created_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    FOREIGN KEY (project_group_id)
        REFERENCES project_group (id)
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS project_group_default_blacklist (
    project_group_id UUID        NOT NULL,

    type_id          INTEGER     NOT NULL,

    created_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    FOREIGN KEY (project_group_id)
        REFERENCES project_group (id)
        ON DELETE CASCADE
);
