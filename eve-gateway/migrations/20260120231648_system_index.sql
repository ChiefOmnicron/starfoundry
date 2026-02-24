CREATE TABLE IF NOT EXISTS system_index (
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
CREATE UNIQUE INDEX IF NOT EXISTS system_index_timestamp_system_id ON system_index (timestamp, system_id);
