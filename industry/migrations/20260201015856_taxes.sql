CREATE TABLE IF NOT EXISTS structure_tax(
    structure_id    UUID    NOT NULL,
    service_type_id INTEGER NOT NULL,
    tax             REAL    NOT NULL,

    FOREIGN KEY (structure_id)
        REFERENCES structure (id)
        ON DELETE CASCADE
);
