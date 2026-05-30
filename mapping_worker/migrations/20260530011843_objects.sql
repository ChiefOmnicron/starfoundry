CREATE TABLE IF NOT EXISTS star (
    star_id         INTEGER NOT NULL,
    type_id         INTEGER NOT NULL,
    system_id       INTEGER NOT NULL,

    radius          INTEGER NOT NULL,
    temperature     FLOAT   NOT NULL,

    PRIMARY KEY(star_id),
    FOREIGN KEY (system_id) REFERENCES system (system_id)
);
CREATE INDEX IF NOT EXISTS star_system_id ON star (system_id);

CREATE TABLE IF NOT EXISTS moon (
    moon_id         INTEGER NOT NULL,
    orbit_id        INTEGER NOT NULL,
    type_id         INTEGER NOT NULL,
    system_id       INTEGER NOT NULL,

    position_x      INTEGER NOT NULL,
    position_y      INTEGER NOT NULL,
    position_z      INTEGER NOT NULL,
    radius          FLOAT   NOT NULL,

    PRIMARY KEY(moon_id),
    FOREIGN KEY (system_id) REFERENCES system (system_id)
);
CREATE INDEX IF NOT EXISTS moon_system_id ON moon (system_id);
CREATE INDEX IF NOT EXISTS moon_orbit_id ON moon (orbit_id);

CREATE TABLE IF NOT EXISTS asteroid_belt (
    asteroid_belt_id    INTEGER NOT NULL,
    orbit_id            INTEGER NOT NULL,
    type_id             INTEGER NOT NULL,
    system_id           INTEGER NOT NULL,

    celestial_index:    INTEGER NOT NULL,
    orbit_index:        INTEGER NOT NULL,

    position_x          INTEGER NOT NULL,
    position_y          INTEGER NOT NULL,
    position_z          INTEGER NOT NULL,

    PRIMARY KEY(moon_id),
    FOREIGN KEY (system_id) REFERENCES system (system_id)
);
CREATE INDEX IF NOT EXISTS asteroid_belt_system_id ON asteroid_belt (system_id);
CREATE INDEX IF NOT EXISTS asteroid_belt_orbit_id ON asteroid_belt (orbit_id);

CREATE TABLE IF NOT EXISTS npc_station (
    npc_station_id      INTEGER NOT NULL,
    orbit_id            INTEGER NOT NULL,
    type_id             INTEGER NOT NULL,
    system_id           INTEGER NOT NULL,

    celestial_index:    INTEGER NOT NULL,
    orbit_index:        INTEGER NOT NULL,

    position_x          INTEGER NOT NULL,
    position_y          INTEGER NOT NULL,
    position_z          INTEGER NOT NULL,

    owner_id            INTEGER NOT NULL,

    PRIMARY KEY(moon_id),
    FOREIGN KEY (system_id) REFERENCES system (system_id)
);
CREATE INDEX IF NOT EXISTS npc_station_system_id ON npc_station (system_id);
CREATE INDEX IF NOT EXISTS npc_station_orbit_id ON npc_station (orbit_id);

CREATE TABLE IF NOT EXISTS planet (
    planet_id       INTEGER NOT NULL,
    orbit_id        INTEGER NOT NULL,
    type_id         INTEGER NOT NULL,
    system_id       INTEGER NOT NULL,

    position_x      INTEGER NOT NULL,
    position_y      INTEGER NOT NULL,
    position_z      INTEGER NOT NULL,
    radius          FLOAT   NOT NULL,

    PRIMARY KEY(planet_id),
    FOREIGN KEY (system_id) REFERENCES system (system_id)
);
CREATE INDEX IF NOT EXISTS planet_system_id ON planet (system_id);
CREATE INDEX IF NOT EXISTS planet_orbit_id ON planet (orbit_id);

CREATE TABLE IF NOT EXISTS planet_moon (
    planet_id       INTEGER NOT NULL,
    moon_id         INTEGER NOT NULL,

    FOREIGN KEY (planet_id) REFERENCES planet (planet_id),
    FOREIGN KEY (moon_id) REFERENCES moon (moon_id)
);
CREATE INDEX IF NOT EXISTS planet_moon_planet_id ON planet_moon (planet_id);

CREATE TABLE IF NOT EXISTS planet_asteroid_belt (
    planet_id           INTEGER NOT NULL,
    asteroid_belt_id    INTEGER NOT NULL,

    FOREIGN KEY (planet_id) REFERENCES planet (planet_id),
    FOREIGN KEY (moon_id) REFERENCES moon (moon_id)
);
CREATE INDEX IF NOT EXISTS planet_moon_planet_id ON planet_moon (planet_id);

CREATE TABLE IF NOT EXISTS stargate (
    stargate_id                 INTEGER NOT NULL,
    destination_system_id       INTEGER NOT NULL,
    destination_stargate_id     INTEGER NOT NULL,
    type_id                     INTEGER NOT NULL,
    system_id                   INTEGER NOT NULL,

    position_x      INTEGER NOT NULL,
    position_y      INTEGER NOT NULL,
    position_z      INTEGER NOT NULL,

    PRIMARY KEY(stargate_id),
    FOREIGN KEY (system_id) REFERENCES system (system_id)
);
CREATE INDEX IF NOT EXISTS stargate_system_id ON stargate (system_id);
