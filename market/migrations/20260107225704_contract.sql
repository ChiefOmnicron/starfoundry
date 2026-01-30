CREATE TABLE IF NOT EXISTS contract (
    contract_id             BIGINT      NOT NULL,
    date_expired            TIMESTAMP   NOT NULL,
    date_issued             TIMESTAMP   NOT NULL,
    issuer_corporation_id   INTEGER     NOT NULL,
    issuer_id               INTEGER     NOT NULL,
    typ                     VARCHAR     NOT NULL,

    title                   VARCHAR,

    -- item exchange
    price                   FLOAT,

    -- auction
    buyout                  FLOAT,

    -- hauling
    collateral              FLOAT,
    days_to_complete        INTEGER,
    for_corporation         BOOLEAN,
    reward                  FLOAT,
    end_location_id         BIGINT,
    start_location_id       BIGINT,
    volume                  FLOAT,

    is_active               BOOLEAN NOT NULL DEFAULT TRUE,

    PRIMARY KEY (contract_id)
);

CREATE TABLE IF NOT EXISTS contract_item (
    contract_id         BIGINT NOT NULL,

    is_included         BOOLEAN NOT NULL,
    is_blueprint_copy   BOOLEAN,

    quantity            BIGINT  NOT NULL,
    -- -1 indicates that the item is a singleton (non-stackable). If the item happens to be a Blueprint, -1 is an Original and -2 is a Blueprint Copy
    record_id           BIGINT  NOT NULL,
    type_id             INTEGER NOT NULL,

    item_id             BIGINT,

    material_efficiency INTEGER,
    time_efficiency     INTEGER,
    runs                INTEGER
);
CREATE INDEX IF NOT EXISTS contract_item_id ON contract_item (contract_id);
CREATE UNIQUE INDEX IF NOT EXISTS contract_item_id_record ON contract_item (contract_id, record_id);
