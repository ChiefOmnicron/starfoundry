--------------------------------------------------------------------------------
--                  Projects
--------------------------------------------------------------------------------

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

--------------------------------------------------------------------------------
--                  SDE
--------------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS items(
    type_id        INTEGER NOT NULL,
    category_id    INTEGER NOT NULL,
    group_id       INTEGER NOT NULL,

    meta_group_id  INTEGER,

    volume         REAL    NOT NULL,

    name           VARCHAR NOT NULL,

    repackaged     INTEGER,

    PRIMARY KEY(type_id)
);

CREATE TABLE IF NOT EXISTS item_reprocessing(
    type_id          INTEGER NOT NULL,
    material_type_id INTEGER NOT NULL,
    quantity         INTEGER NOT NULL
);
CREATE INDEX IF NOT EXISTS type_ids ON item_reprocessing(type_id);

CREATE TABLE IF NOT EXISTS systems(
    region_id          INTEGER NOT NULL,
    constellation_id   INTEGER NOT NULL,
    system_id          INTEGER NOT NULL,

    region_name        VARCHAR NOT NULL,
    constellation_name VARCHAR NOT NULL,
    system_name        VARCHAR NOT NULL,

    security           REAL    NOT NULL,

    PRIMARY KEY(system_id)
);
CREATE INDEX IF NOT EXISTS systems_ids ON systems(region_id, constellation_id, system_id);

CREATE TABLE IF NOT EXISTS blueprint_json(
    btype_id INTEGER NOT NULL,
    ptype_id INTEGER NOT NULL,
    data     JSON    NOT NULL
);
CREATE INDEX IF NOT EXISTS blueprint_json_typeid ON blueprint_json (btype_id, ptype_id);

CREATE TABLE IF NOT EXISTS blueprint_dependencies(
    btype_id   INTEGER   NOT NULL,
    ptype_id   INTEGER   NOT NULL,
    time       INTEGER   NOT NULL,
    depends_on INTEGER[] NOT NULL
);
CREATE INDEX IF NOT EXISTS blueprint_dependencies ON blueprint_json (btype_id, ptype_id);

CREATE TABLE IF NOT EXISTS industry_index(
    id                UUID      NOT NULL DEFAULT gen_random_uuid(),
    timestamp         TIMESTAMP NOT NULL NOW(),

    system_id         INTEGER   NOT NULL,
    manufacturing     REAL      NOT NULL,
    copying           REAL      NOT NULL,
    invention         REAL      NOT NULL,
    reaction          REAL      NOT NULL,
    research_time     REAL      NOT NULL,
    research_material REAL      NOT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS industry_index_timestamp_system_id ON industry_index (timestamp, system_id);

CREATE TABLE IF NOT EXISTS industry_jobs(
    -- Asset ID of the blueprint
    blueprint_id             BIGINT            NOT NULL,
    -- Location ID of the location from which the blueprint was installed. Normally a station ID, but can also be an asset (e.g. container) or corporation facility
    blueprint_location_id    BIGINT            NOT NULL,
    blueprint_type_id        INTEGER           NOT NULL,

    facility_id              BIGINT            NOT NULL,
    installer_id             INTEGER           NOT NULL,
    character_corporation_id INTEGER           NOT NULL,

    job_id                   INTEGER           NOT NULL,

    runs                     DOUBLE PRECISION  NOT NULL,
    cost                     REAL              NOT NULL,

    end_date                 VARCHAR           NOT NULL,
    activity                 INDUSTRY_ACTIVITY NOT NULL,

    is_delivered             BOOLEAN           NOT NULL DEFAULT false,
    ignore                   BOOLEAN           NOT NULL DEFAULT false,

    PRIMARY KEY (job_id)
);
CREATE INDEX IF NOT EXISTS industry_jobs_installer_id
ON industry_jobs (installer_id);

CREATE TABLE IF NOT EXISTS system_names(
    system_id BIGINT  NOT NULL,
    security  REAL    NOT NULL,
    name      VARCHAR NOT NULL
);
CREATE INDEX IF NOT EXISTS system_names_system_id ON system_names(system_id);

CREATE TABLE IF NOT EXISTS wallet_character(
    id          BIGINT  NOT NULL,
    receiver    BIGINT  NOT NULL,
    sender      BIGINT  NOT NULL,

    character   BIGINT  NOT NULL,

    amount      FLOAT   NOT NULL,
    balance     FLOAT   NOT NULL,

    date        VARCHAR NOT NULL,
    ref_type    VARCHAR NOT NULL,

    reason      VARCHAR,
    context_id  BIGINT,

    PRIMARY KEY(id),
    FOREIGN KEY (character)
        REFERENCES characters(character_id)
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS wallet_corporation(
    id          BIGINT  NOT NULL,
    receiver    BIGINT  NOT NULL,
    sender      BIGINT  NOT NULL,

    corporation BIGINT  NOT NULL,
    division    INTEGER NOT NULL,

    amount      FLOAT   NOT NULL,
    balance     FLOAT   NOT NULL,

    date        VARCHAR NOT NULL,
    ref_type    VARCHAR NOT NULL,

    reason      VARCHAR,
    context_id  BIGINT,

    PRIMARY KEY(id)
);

CREATE TABLE IF NOT EXISTS job_detection_ignore_hangars(
    location_id BIGINT NOT NULL,

    PRIMARY KEY(location_id)
);
CREATE UNIQUE INDEX job_detection_ignore_hangars_location ON job_detection_ignore_hangars(location_id);
