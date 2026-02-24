CREATE TABLE IF NOT EXISTS asset (
    owner_id            INTEGER NOT NULL,

    item_id             BIGINT  NOT NULL,
    type_id             BIGINT  NOT NULL,
    location_id         BIGINT  NOT NULL,
    location_flag       VARCHAR NOT NULL,

    location_type       VARCHAR NOT NULL,
    quantity            INTEGER NOT NULL,

    is_singleton        BOOLEAN NOT NULL,
    is_blueprint_copy   BOOLEAN,

    PRIMARY KEY (item_id)
);
CREATE INDEX IF NOT EXISTS asset_owner ON asset (owner_id);
