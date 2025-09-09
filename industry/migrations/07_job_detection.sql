CREATE TABLE IF NOT EXISTS industry_job (
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
CREATE INDEX IF NOT EXISTS industry_job_installer_id ON industry_job (installer_id);

CREATE TABLE IF NOT EXISTS job_detection_log (
    id           UUID      NOT NULL DEFAULT uuidv7(),
    -- type id of the item that is build
    type_id      INTEGER   NOT NULL,
    -- eve id of the job
    job_id       INTEGER   NOT NULL,

    -- TODO: why
    time_changed TIMESTAMP NOT NULL DEFAULT NOW(),

    -- id of the project the job is assigned to
    -- can be null if it is unassigned
    project_id   UUID,

    PRIMARY KEY (id)
);
CREATE UNIQUE INDEX IF NOT EXISTS job_detection_log_job_id ON job_detection_log (job_id);
