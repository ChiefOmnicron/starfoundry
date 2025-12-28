CREATE TABLE IF NOT EXISTS market_price (
    type_id        INTEGER          NOT NULL,
    adjusted_price DOUBLE PRECISION NOT NULL,
    average_price  DOUBLE PRECISION NOT NULL,

    PRIMARY KEY(type_id)
);

-- latest list of orders, includes buy and sell
CREATE TABLE IF NOT EXISTS market_order_latest (
    -- ID of the station the order is located in
    structure_id    BIGINT    NOT NULL,
    -- Id from eve
    order_id        BIGINT    NOT NULL,
    -- ID of the region the order is located in
    region_id       INTEGER    NOT NULL,
    -- If the entry is a buy or sell order
    is_buy          BOOLEAN   NOT NULL,

    -- TypeId of the item
    type_id         INTEGER   NOT NULL,
    -- Amount of items that are remaining in this order
    remaining       INTEGER   NOT NULL,
    -- Price of the item
    price           FLOAT     NOT NULL,
    -- Date the order expires naturally
    expires         TIMESTAMP NOT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS market_order_latest_orderid ON market_order_latest(order_id);
CREATE INDEX IF NOT EXISTS market_order_latest_structureid_typeid_isbuy ON market_order_latest(structure_id, type_id, is_buy);
