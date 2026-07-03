CREATE TABLE IF NOT EXISTS tag (
    id          UUID        NOT NULL DEFAULT uuidv7(),

    owner_id    INTEGER     NOT NULL,

    content     VARCHAR     NOT NULL,
    color       VARCHAR     NOT NULL,
    typ         VARCHAR     NOT NULL,

    PRIMARY KEY(id)
);

CREATE TABLE IF NOT EXISTS tag_auto (
    id          UUID        NOT NULL DEFAULT uuidv7(),
    tag_id      UUID        NOT NULL,

    option      VARCHAR     NOT NULL,
    compare     VARCHAR     NOT NULL,
    value       VARCHAR     NOT NULL,

    FOREIGN KEY (tag_id)
        REFERENCES tag(id)
        ON DELETE CASCADE
);
