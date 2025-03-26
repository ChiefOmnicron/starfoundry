-- temporary blueprint table until one with more info is created
-- only used for bpc_stock
CREATE TABLE IF NOT EXISTS blueprints_temp(
    type_id  INTEGER NOT NULL,
    max_runs INTEGER NOT NULL,

    PRIMARY KEY (type_id)
);
