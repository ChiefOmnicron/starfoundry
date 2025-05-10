DO
$$
BEGIN
    IF NOT EXISTS (
        SELECT *
        FROM pg_type typ
        INNER JOIN pg_namespace nsp ON nsp.oid = typ.typnamespace
        WHERE nsp.nspname = current_schema()
        AND typ.typname = 'bonus_modifier'
    ) THEN
        CREATE TYPE BONUS_MODIFIER AS ENUM (
            'MANUFACTURE_MATERIAL',
            'MANUFACTURE_TIME',
            'REACTION_MATERIAL',
            'REACTION_TIME'
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
        AND typ.typname = 'system_security'
    ) THEN
        CREATE TYPE SYSTEM_SECURITY AS ENUM (
            'NULLSEC',
            'LOWSEC',
            'HIGHSEC'
        );
    END IF;
END;
$$
LANGUAGE plpgsql;

CREATE TABLE IF NOT EXISTS structure (
    id           UUID            NOT NULL DEFAULT gen_random_uuid(),
    owner        INTEGER         NOT NULL,
    structure_id BIGINT          NOT NULL,

    system_id    INTEGER         NOT NULL,

    -- Structure Type id
    type_id      INTEGER         NOT NULL,
    rigs         INTEGER[]       NOT NULL DEFAULT ARRAY[]::INTEGER[],
    services     INTEGER[]       NOT NULL DEFAULT ARRAY[]::INTEGER[],

    security     SYSTEM_SECURITY NOT NULL,

    name         VARCHAR(128)    NOT NULL,

    created_at   TIMESTAMPTZ     NOT NULL DEFAULT NOW(),
    updated_at   TIMESTAMPTZ     NOT NULL DEFAULT NOW(),

    PRIMARY KEY (id, owner),

    FOREIGN KEY (owner)
        REFERENCES character (character_id)
        ON DELETE NO ACTION
);
CREATE UNIQUE INDEX IF NOT EXISTS structure_id ON structure (id);

CREATE TABLE IF NOT EXISTS structure_project_group (
    structure_id     UUID        NOT NULL,
    project_group_id UUID        NOT NULL,

    created_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    PRIMARY KEY (structure_id, project_group_id),

    FOREIGN KEY (structure_id)
        REFERENCES structure (id)
        ON DELETE NO ACTION,

    FOREIGN KEY (project_group_id)
        REFERENCES project_group (id)
        ON DELETE NO ACTION
);

CREATE TABLE IF NOT EXISTS structure_group (
    id            UUID        NOT NULL DEFAULT gen_random_uuid(),
    owner         INTEGER     NOT NULL,

    name          VARCHAR     NOT NULL,

    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    PRIMARY KEY (id, owner),

    FOREIGN KEY (owner)
        REFERENCES character (character_id)
        ON DELETE CASCADE
);

-- maps structure_group and structure together
CREATE TABLE IF NOT EXISTS structure_group_structure (
    structure_group_id UUID NOT NULL,
    structure_id       UUID NOT NULL,

    FOREIGN KEY (structure_id)
        REFERENCES structure (id)
        ON DELETE NO ACTION
);

CREATE TABLE IF NOT EXISTS structure_dynamic_group (
    id         UUID        NOT NULL DEFAULT gen_random_uuid(),
    owner      INTEGER     NOT NULL,

    name       VARCHAR     NOT NULL,

    group_ids  UUID[]      NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    PRIMARY KEY (id, owner),

    FOREIGN KEY (owner)
        REFERENCES character (character_id)
        ON DELETE NO ACTION
);
