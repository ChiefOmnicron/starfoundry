CREATE TABLE IF NOT EXISTS stock_blueprints (
    id          UUID    NOT NULL DEFAULT gen_random_uuid(),
    name        VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    owner       INTEGER NOT NULL,

    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS stock_blueprint_thresholds (
    id                 UUID    NOT NULL DEFAULT gen_random_uuid(),
    blueprint_stock_id UUID    NOT NULL,

    type_id            INTEGER NOT NULL,
    want               INTEGER NOT NULL,
    critical           INTEGER NOT NULL,

    min_runs           INTEGER NOT NULL DEFAULT 0,
    min_me             INTEGER NOT NULL DEFAULT 0,
    min_te             INTEGER NOT NULL DEFAULT 0,

    PRIMARY KEY (id),
    FOREIGN KEY (blueprint_stock_id)
        REFERENCES bpc_stocks(id)
        ON DELETE CASCADE
);
