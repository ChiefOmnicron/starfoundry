-- latest list of orders, includes buy and sell
CREATE TABLE IF NOT EXISTS market_private_order (
    -- either a character_id or a corporation_id
    issuer_id       INTEGER   NOT NULL,

    -- ID of the station the order is located in
    structure_id    BIGINT    NOT NULL,
    -- Id from eve
    order_id        BIGINT    NOT NULL,
    -- If the entry is a buy or sell order
    is_buy          BOOLEAN   NOT NULL,

    -- TypeId of the item
    type_id         INTEGER   NOT NULL,
    -- Amount of items that are remaining in this order
    remaining       INTEGER   NOT NULL,
    -- Price of the item
    price           FLOAT     NOT NULL,
    -- Date the order expires naturally
    expires         TIMESTAMP NOT NULL,

    PRIMARY KEY (order_id)
);
CREATE INDEX IF NOT EXISTS market_private_order_issuer ON market_private_order(issuer_id);

CREATE TABLE IF NOT EXISTS market_private_order_history (
    order_id    BIGINT      NOT NULL,
    remaining   INTEGER     NOT NULL,

    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS market_private_order_history_order ON market_private_order_history (order_id);
CREATE UNIQUE INDEX IF NOT EXISTS market_private_order_history_order_remaining ON market_private_order_history (order_id, remaining);
