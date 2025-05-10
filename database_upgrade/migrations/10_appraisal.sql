CREATE TABLE IF NOT EXISTS appraisal (
    id             UUID                        NOT NULL DEFAULT gen_random_uuid(),

    structure_id   BIGINT                      NOT NULL,
    code           VARCHAR(10),

    market         BIGINT                      NOT NULL DEFAULT 60003760,
    price_modifier SMALLINT                    NOT NULL DEFAULT 100,

    comment        VARCHAR(1024),

    raw            VARCHAR,

    created_at     TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT (NOW() AT TIME ZONE ('utc')),

    PRIMARY KEY(id)
);
CREATE UNIQUE INDEX IF NOT EXISTS appraisal_code ON appraisal(code);

CREATE TABLE IF NOT EXISTS appraisal_item (
    id           UUID    NOT NULL DEFAULT gen_random_uuid(),
    appraisal_id UUID    NOT NULL,

    type_id      INTEGER NOT NULL,
    quantity     BIGINT  NOT NULL,

    buy          UUID    NOT NULL,
    sell         UUID    NOT NULL,

    low_data     BOOLEAN NOT NULL DEFAULT FALSE,

    PRIMARY KEY(id),

    FOREIGN KEY (appraisal_id)
        REFERENCES appraisal (id)
        ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS appraisal_item_appraisalid ON appraisal_item(appraisal_id);

CREATE TABLE IF NOT EXISTS appraisal_invalid (
    id           UUID    NOT NULL DEFAULT gen_random_uuid(),
    appraisal_id UUID    NOT NULL,

    raw          VARCHAR NOT NULL,

    PRIMARY KEY(id),

    FOREIGN KEY (appraisal_id)
        REFERENCES appraisal (id)
        ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS appraisal_invalid_appraisalid ON appraisal_invalid(appraisal_id);

CREATE TABLE IF NOT EXISTS appraisal_market_price (
    id           UUID             NOT NULL DEFAULT gen_random_uuid(),
    appraisal_id UUID             NOT NULL,
    type_id      INTEGER          NOT NULL,

    is_buy       BOOLEAN          NOT NULL,

    min          DOUBLE PRECISION NOT NULL,
    max          DOUBLE PRECISION NOT NULL,
    avg          DOUBLE PRECISION NOT NULL,
    total_orders BIGINT           NOT NULL,

    PRIMARY KEY(id),

    FOREIGN KEY (appraisal_id)
        REFERENCES appraisal (id)
        ON DELETE CASCADE
);
