-- Add migration script here
-- updates the updated_at field automatically
CREATE OR REPLACE FUNCTION trigger_set_updated_at()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TABLE IF NOT EXISTS product (
    id                      UUID            NOT NULL    DEFAULT uuidv7(),

    category                VARCHAR         NOT NULL    DEFAULT 'Uncategorized',
    name                    VARCHAR(100)    NOT NULL,
    price                   BIGINT          NOT NULL    CONSTRAINT positive_price CHECK (price >= 0),
    image_type              VARCHAR         NOT NULL,
    image_type_id           INTEGER         NOT NULL,
    content                 JSONB           NOT NULL,
    hidden                  BOOLEAN         NOT NULL    DEFAULT FALSE,
    description             VARCHAR(10000),
    tags                    VARCHAR[],
    additional_products     UUID[],

    message                 VARCHAR,
    delivery_time           VARCHAR         NOT NULL,
    whitelist               INTEGER[]       NOT NULL    DEFAULT ARRAY[]::INTEGER[],
    blacklist               INTEGER[]       NOT NULL    DEFAULT ARRAY[]::INTEGER[],

    delivery_location       INTEGER[]       NOT NULL    DEFAULT ARRAY[]::INTEGER[],

    created_at              TIMESTAMPTZ     NOT NULL    DEFAULT NOW(),
    updated_at              TIMESTAMPTZ     NOT NULL    DEFAULT NOW(),

    PRIMARY KEY(id)
);
CREATE OR REPLACE TRIGGER set_updated_at
    AFTER INSERT ON product
    EXECUTE FUNCTION trigger_set_updated_at();

CREATE TABLE IF NOT EXISTS order_info(
    id                      UUID            NOT NULL    DEFAULT uuidv7(),

    character_id            INTEGER         NOT NULL,
    quantity                INTEGER         NOT NULL    DEFAULT 1,
    status                  VARCHAR         NOT NULL    DEFAULT 'OPEN', -- IN_PROGRESS, DELIVERED
    delivery_location       VARCHAR         NOT NULL,

    comment                 VARCHAR,

    created_at              TIMESTAMPTZ     NOT NULL    DEFAULT NOW(),
    updated_at              TIMESTAMPTZ     NOT NULL    DEFAULT NOW(),

    PRIMARY KEY(id)
);
CREATE OR REPLACE TRIGGER set_updated_at
    AFTER INSERT ON order_info
    EXECUTE FUNCTION trigger_set_updated_at();

CREATE TABLE IF NOT EXISTS order_product (
    id                      UUID            NOT NULL    DEFAULT uuidv7(),
    order_id                UUID            NOT NULL,

    name                    VARCHAR(100)    NOT NULL,
    price                   BIGINT          NOT NULL    CONSTRAINT positive_price CHECK (price >= 0),
    image_type              VARCHAR         NOT NULL,
    image_type_id           INTEGER         NOT NULL,
    content                 JSONB           NOT NULL,

    is_additional           BOOLEAN         NOT NULL DEFAULT FALSE,

    PRIMARY KEY(id),
    FOREIGN KEY(order_id) REFERENCES order_info(id) ON DELETE CASCADE
);

-- for the worker
CREATE TABLE buildcost_history (
    id                      UUID        NOT NULL DEFAULT gen_random_uuid(),

    product_id              UUID        NOT NULL,
    bpc                     REAL        NOT NULL,
    market                  REAL        NOT NULL,
    manufacturing           REAL        NOT NULL,
    manufacturing_market    REAL        NOT NULL,
    total                   REAL        NOT NULL,
    sell_price              INTEGER     NOT NULL,
    date                    DATE        NOT NULL DEFAULT NOW(),

    PRIMARY KEY (id)
);
CREATE INDEX IF NOT EXISTS buildcost_history_product ON buildcost_history(product_id);
CREATE INDEX IF NOT EXISTS buildcost_history_date ON buildcost_history(date);
