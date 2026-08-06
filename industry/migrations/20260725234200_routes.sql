CREATE TABLE IF NOT EXISTS route(
    id                  UUID            NOT NULL DEFAULT uuidv7(),

    name                VARCHAR(100)    NOT NULL,
    typ                 VARCHAR         NOT NULL,

    start_structure_id  UUID,
    end_structure_id    UUID,

    PRIMARY KEY(id),

    FOREIGN KEY (start_structure_id)
        REFERENCES structure(id)
        ON DELETE SET NULL,

    FOREIGN KEY (end_structure_id)
        REFERENCES structure(id)
        ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS route_jump_route(
    id              UUID    NOT NULL DEFAULT uuidv7(),
    route_id        UUID    NOT NULL,

    fuel_usage      INTEGER NOT NULL,

    FOREIGN KEY (route_id)
        REFERENCES route(id)
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS route_hauling_route(
    id                  UUID    NOT NULL DEFAULT uuidv7(),
    route_id            UUID    NOT NULL,

    fuel_usage          INTEGER NOT NULL,
    max_cargo_m3        INTEGER NOT NULL,

    FOREIGN KEY (route_id)
        REFERENCES route(id)
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS route_hauling_service(
    id                  UUID    NOT NULL DEFAULT uuidv7(),
    route_id            UUID    NOT NULL,

    contract_to         VARCHAR NOT NULL,
    price_per_m3        INTEGER NOT NULL,
    max_cargo_m3        INTEGER NOT NULL,
    collateral_percent  INTEGER NOT NULL,

    FOREIGN KEY (route_id)
        REFERENCES route(id)
        ON DELETE CASCADE
);
