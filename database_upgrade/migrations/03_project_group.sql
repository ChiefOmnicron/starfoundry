CREATE TABLE IF NOT EXISTS project_group (
    id          UUID          NOT NULL DEFAULT gen_random_uuid(),

    owner       INTEGER       NOT NULL,

    name        VARCHAR(128)  NOT NULL,
    description VARCHAR(2048),

    created_at   TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at   TIMESTAMPTZ  NOT NULL DEFAULT NOW(),

    PRIMARY KEY (id)
);

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
