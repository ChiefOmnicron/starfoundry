CREATE TABLE IF NOT EXISTS project_tag (
    id          UUID    NOT NULL DEFAULT uuidv7(),
    project_id  UUID    NOT NULL,
    tag_id      UUID    NOT NULL,

    FOREIGN KEY (project_id)
        REFERENCES project (id)
        ON DELETE CASCADE,

    FOREIGN KEY (tag_id)
        REFERENCES tag (id)
        ON DELETE CASCADE
);
