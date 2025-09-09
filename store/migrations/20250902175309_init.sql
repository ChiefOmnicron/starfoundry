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
    uuid                    UUID            NOT NULL    DEFAULT gen_random_uuid(),

    category                VARCHAR         NOT NULL    DEFAULT 'Uncategorized',
    name                    VARCHAR(100)    NOT NULL,
    price                   BIGINT          NOT NULL    CONSTRAINT positive_price CHECK (price >= 0),
    image_type              VARCHAR         NOT NULL,
    image_type_id           INTEGER         NOT NULL,
    content                 JSONB           NOT NULL,
    description             VARCHAR(10000),
    tags                    VARCHAR[],
    additional_products     UUID[],

    created_at              TIMESTAMPTZ     NOT NULL    DEFAULT NOW(),
    updated_at              TIMESTAMPTZ     NOT NULL    DEFAULT NOW(),

    PRIMARY KEY(uuid)
);
CREATE OR REPLACE TRIGGER set_updated_at
    AFTER INSERT ON product
    EXECUTE FUNCTION trigger_set_updated_at();

CREATE TABLE IF NOT EXISTS order_info(
    uuid                    UUID            NOT NULL    DEFAULT gen_random_uuid(),

    character_id            INTEGER         NOT NULL,
    quantity                INTEGER         NOT NULL    DEFAULT 1,
    status                  VARCHAR         NOT NULL    DEFAULT 'OPEN', -- IN_PROGRESS, DELIVERED
    delivery_location       VARCHAR         NOT NULL,

    comment                 VARCHAR,

    created_at              TIMESTAMPTZ     NOT NULL    DEFAULT NOW(),
    updated_at              TIMESTAMPTZ     NOT NULL    DEFAULT NOW(),

    PRIMARY KEY(uuid)
);
CREATE OR REPLACE TRIGGER set_updated_at
    AFTER INSERT ON order_info
    EXECUTE FUNCTION trigger_set_updated_at();

CREATE TABLE IF NOT EXISTS order_product (
    uuid                    UUID            NOT NULL    DEFAULT gen_random_uuid(),
    order_uuid              UUID            NOT NULL,

    name                    VARCHAR(100)    NOT NULL,
    price                   BIGINT          NOT NULL    CONSTRAINT positive_price CHECK (price >= 0),
    image_type              VARCHAR         NOT NULL,
    image_type_id           INTEGER         NOT NULL,
    content                 JSONB           NOT NULL,

    is_additional           BOOLEAN         NOT NULL DEFAULT FALSE,

    PRIMARY KEY(uuid),
    FOREIGN KEY(order_uuid) REFERENCES order_info(uuid) ON DELETE CASCADE
);
