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

CREATE TABLE IF NOT EXISTS structure_dogma (
  ptype_id    INTEGER         NOT NULL,
  modifier    BONUS_MODIFIER  NOT NULL,
  amount      FLOAT           NOT NULL,
  categories  INTEGER[]       NOT NULL,
  groups      INTEGER[]       NOT NULL
);

CREATE TABLE IF NOT EXISTS structures (
    id           UUID            NOT NULL DEFAULT gen_random_uuid(),
    owner        INTEGER         NOT NULL,
    structure_id BIGINT          NOT NULL,

    system_id    INTEGER         NOT NULL,

    -- Structure Type id
    type_id      INTEGER         NOT NULL,
    rigs         INTEGER[]       NOT NULL DEFAULT ARRAY[]::INTEGER[],
    services     INTEGER[]       NOT NULL DEFAULT ARRAY[]::INTEGER[],

    security     SYSTEM_SECURITY NOT NULL,

    name         VARCHAR         NOT NULL,

    PRIMARY KEY (id, owner),

    FOREIGN KEY (owner)
        REFERENCES characters (character_id)
        ON DELETE NO ACTION
);
CREATE UNIQUE INDEX IF NOT EXISTS structures_id ON structures(id);

CREATE TABLE IF NOT EXISTS structure_project_group (
    structure_id     UUID NOT NULL,
    project_group_id UUID NOT NULL,

    PRIMARY KEY (structure_id, project_group_id),

    FOREIGN KEY (structure_id)
        REFERENCES structures (id)
        ON DELETE CASCADE,

    FOREIGN KEY (project_group_id)
        REFERENCES project_groups (id)
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS structure_groups (
    id            UUID    NOT NULL DEFAULT gen_random_uuid(),
    owner         INTEGER NOT NULL,

    name          VARCHAR NOT NULL,

    structure_ids UUID[]  NOT NULL,

    PRIMARY KEY (id, owner),

    FOREIGN KEY (owner)
        REFERENCES characters (character_id)
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS structure_dynamic_groups (
    id        UUID      NOT NULL DEFAULT gen_random_uuid(),
    owner     INTEGER   NOT NULL,

    name      VARCHAR   NOT NULL,

    group_ids UUID[]    NOT NULL,

    PRIMARY KEY (id, owner),

    FOREIGN KEY (owner)
        REFERENCES characters (character_id)
        ON DELETE CASCADE
);

-- possible rigs for structures
CREATE TABLE IF NOT EXISTS structure_rigs(
    type_id    INTEGER   NOT NULL,
    structures INTEGER[] NOT NULL,

    PRIMARY KEY(type_id)
);
