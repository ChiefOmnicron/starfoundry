CREATE TABLE IF NOT EXISTS structure_character (
    structure_id      UUID    NOT NULL,
    character         INTEGER NOT NULL,

    marked_for_delete BOOLEAN NOT NULL DEFAULT FALSE,

    PRIMARY KEY(structure_id)
);
CREATE INDEX IF NOT EXISTS structure_character_character ON structure_character (character);

INSERT INTO structure_character (structure_id, character)
SELECT id, owner FROM structure;
