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
CREATE UNIQUE INDEX IF NOT EXISTS industry_job_job_id ON industry_job(job_id);
