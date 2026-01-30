CREATE TABLE IF NOT EXISTS asset_blueprint (
    owner_id            INTEGER NOT NULL,

    item_id             BIGINT  NOT NULL,
    location_id         BIGINT  NOT NULL,
    type_id             BIGINT  NOT NULL,

    location_flag       VARCHAR NOT NULL,
    material_efficiency INTEGER NOT NULL,
    time_efficiency     INTEGER NOT NULL,
    quantity            INTEGER NOT NULL,
    runs                INTEGER NOT NULL,

    PRIMARY KEY (item_id)
);
CREATE INDEX IF NOT EXISTS asset_blueprint_owner ON asset_blueprint (owner_id);
