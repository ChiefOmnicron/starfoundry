CREATE TABLE IF NOT EXISTS stock_blueprint (
    id          UUID          NOT NULL DEFAULT uuidv7(),
    name        VARCHAR(128)  NOT NULL,
    description VARCHAR(2048) NOT NULL,
    owner       INTEGER       NOT NULL,

    created_at  TIMESTAMPTZ   NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ   NOT NULL DEFAULT NOW(),

    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS stock_blueprint_threshold (
    id                 UUID        NOT NULL DEFAULT uuidv7(),
    blueprint_stock_id UUID        NOT NULL,

    type_id            INTEGER     NOT NULL,
    want               INTEGER     NOT NULL,
    critical           INTEGER     NOT NULL,

    min_runs           INTEGER     NOT NULL DEFAULT 0,
    min_me             INTEGER     NOT NULL DEFAULT 0,
    min_te             INTEGER     NOT NULL DEFAULT 0,

    created_at         TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at         TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    PRIMARY KEY (id),

    FOREIGN KEY (blueprint_stock_id)
        REFERENCES stock_blueprint (id)
        ON DELETE CASCADE
);
