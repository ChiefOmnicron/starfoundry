CREATE TABLE IF NOT EXISTS appraisal(
    code            VARCHAR(10)     NOT NULL,

    market_id       BIGINT          NOT NULL,
    price_modifier  SMALLINT        NOT NULL DEFAULT 100,
    mode            VARCHAR         NOT NULL DEFAULT 'APPRAISAL',

    market_info     jsonb           NOT NULL,

    comment         VARCHAR(1024),
    invalid_items   VARCHAR,
    raw             VARCHAR,

    created_at     TIMESTAMPTZ      NOT NULL DEFAULT NOW(),

    PRIMARY KEY(code)
);
