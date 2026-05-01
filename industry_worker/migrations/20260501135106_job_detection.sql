CREATE TABLE job_detection_log(
    id                  UUID        NOT NULL DEFAULT uuidv7(),
    job_id              INTEGER     NOT NULL,
    type_id             INTEGER     NOT NULL,
    result              VARCHAR     NOT NULL,

    project_id          UUID,

    PRIMARY KEY(id)
);
CREATE UNIQUE INDEX IF NOT EXISTS job_detection_log_job_id ON job_detection_log(job_id);