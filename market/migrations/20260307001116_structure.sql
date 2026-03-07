CREATE TABLE IF NOT EXISTS structure (
    id              UUID       NOT NULL DEFAULT uuidv7(),

    main_character  INTEGER NOT NULL,
    character_id    INTEGER NOT NULL,
    structure_id    BIGINT  NOT NULL,
    source          VARCHAR NOT NULL
);
