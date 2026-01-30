CREATE TABLE IF NOT EXISTS tmp_mappings (
    source_uuid UUID    NOT NULL,
    target_uuid UUID    NOT NULL,

    environment VARCHAR NOT NULL,

    PRIMARY KEY (source_uuid)
);
