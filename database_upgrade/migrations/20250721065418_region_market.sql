-- Add migration script here
ALTER TABLE market_order_latest ADD COLUMN region_id INTEGER NOT NULL DEFAULT 0;
