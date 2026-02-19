CREATE TABLE IF NOT EXISTS market_order_history (
    order_id    BIGINT      NOT NULL,
    remaining   INTEGER     NOT NULL,

    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS market_order_history_order ON market_order_history (order_id);
CREATE UNIQUE INDEX IF NOT EXISTS market_order_history_order_remaining ON market_order_history (order_id, remaining);
