ALTER TABLE order_info ADD COLUMN IF NOT EXISTS expected_delivery_date TIMESTAMPTZ;
ALTER TABLE order_info ADD COLUMN IF NOT EXISTS sf_industry_link VARCHAR;
