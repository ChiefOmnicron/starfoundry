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

CREATE TABLE IF NOT EXISTS item (
    type_id        INTEGER NOT NULL,
    category_id    INTEGER NOT NULL,
    group_id       INTEGER NOT NULL,

    meta_group_id  INTEGER,

    volume         REAL    NOT NULL,

    name           VARCHAR NOT NULL,

    repackaged     INTEGER,

    PRIMARY KEY(type_id)
);

CREATE TABLE IF NOT EXISTS item_reprocessing (
    type_id          INTEGER NOT NULL,
    material_type_id INTEGER NOT NULL,
    quantity         INTEGER NOT NULL
);
CREATE INDEX IF NOT EXISTS type_id ON item_reprocessing (type_id);

CREATE TABLE IF NOT EXISTS system (
    region_id          INTEGER NOT NULL,
    constellation_id   INTEGER NOT NULL,
    system_id          INTEGER NOT NULL,

    region_name        VARCHAR NOT NULL,
    constellation_name VARCHAR NOT NULL,
    system_name        VARCHAR NOT NULL,

    security           REAL    NOT NULL,

    PRIMARY KEY(system_id)
);
CREATE INDEX IF NOT EXISTS systems_id ON system (region_id, constellation_id, system_id);

CREATE TABLE IF NOT EXISTS blueprint_json(
    btype_id INTEGER NOT NULL,
    ptype_id INTEGER NOT NULL,
    data     JSON    NOT NULL
);
CREATE INDEX IF NOT EXISTS blueprint_json_typeid ON blueprint_json (btype_id, ptype_id);

CREATE TABLE IF NOT EXISTS blueprint_dependency (
    btype_id   INTEGER   NOT NULL,
    ptype_id   INTEGER   NOT NULL,
    time       INTEGER   NOT NULL,
    depends_on INTEGER[] NOT NULL
);
CREATE INDEX IF NOT EXISTS blueprint_dependency ON blueprint_json (btype_id, ptype_id);

CREATE TABLE IF NOT EXISTS industry_index (
    id                UUID      NOT NULL DEFAULT uuidv7(),
    timestamp         TIMESTAMP NOT NULL DEFAULT NOW(),

    system_id         INTEGER   NOT NULL,
    manufacturing     REAL      NOT NULL,
    copying           REAL      NOT NULL,
    invention         REAL      NOT NULL,
    reaction          REAL      NOT NULL,
    research_time     REAL      NOT NULL,
    research_material REAL      NOT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS industry_index_timestamp_system_id ON industry_index (timestamp, system_id);

-- possible rigs for structures
CREATE TABLE IF NOT EXISTS structure_rig (
    type_id    INTEGER   NOT NULL,
    structures INTEGER[] NOT NULL,

    PRIMARY KEY(type_id)
);

CREATE TABLE IF NOT EXISTS structure_service (
    structure_type_id INTEGER   NOT NULL,
    service_slots     INTEGER   NOT NULL,
    service_type_ids  INTEGER[] NOT NULL,

    PRIMARY KEY(structure_type_id)
);

CREATE TABLE IF NOT EXISTS structure_dogma (
  ptype_id    INTEGER         NOT NULL,
  modifier    BONUS_MODIFIER  NOT NULL,
  amount      FLOAT           NOT NULL,
  categories  INTEGER[]       NOT NULL,
  groups      INTEGER[]       NOT NULL
);

-- temporary blueprint table until one with more info is created
-- only used for bpc_stock
-- TODO: remove sooner rather than later
CREATE TABLE IF NOT EXISTS blueprints_temp(
    type_id  INTEGER NOT NULL,
    max_runs INTEGER NOT NULL,

    PRIMARY KEY (type_id)
);
