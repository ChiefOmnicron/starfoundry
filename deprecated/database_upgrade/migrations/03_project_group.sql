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
    group_id          UUID,

    -- character id
    character_id      INTEGER     NOT NULL,

    accepted          BOOLEAN     NOT NULL DEFAULT FALSE,

    permission        INTEGER     NOT NULL,

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

CREATE TABLE IF NOT EXISTS project_group_permission (
    bit     INTEGER NOT NULL,
    name    VARCHAR NOT NULL,
    comment VARCHAR NOT NULL,

    PRIMARY KEY (bit)
);
INSERT INTO project_group_permission (bit, name, comment) VALUES
(1, 'Owner', 'Owner of the project group, has all permissions'),
(2, 'Read Group', 'Permission to read everything from the group'),
(4, 'Write Project', 'Allows to use the group to create new projects'),
(8, 'Write Structure', 'Allows to add additional structures to the group'),
(16, 'Write Defaults', 'Allows to update the group defaults'),
(32, 'Write Members', 'Allows to add additional members to the group'),
(64, 'Write Group', 'Allows to perform updates on the group itself')
ON CONFLICT DO NOTHING;
