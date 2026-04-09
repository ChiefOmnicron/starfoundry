ALTER TABLE market_order_latest ADD COLUMN IF NOT EXISTS virtual_remaining INTEGER NOT NULL DEFAULT 0;
ALTER TABLE market_order_latest ALTER COLUMN virtual_remaining DROP DEFAULT;
