DO
$$
BEGIN
    IF NOT EXISTS (
        SELECT *
        FROM pg_type typ
        INNER JOIN pg_namespace nsp ON nsp.oid = typ.typnamespace
        WHERE nsp.nspname = current_schema()
        AND typ.typname = 'notification_target'
    ) THEN
        CREATE TYPE NOTIFICATION_TARGET AS ENUM (
            'DISCORD',
            'JSON'
        );
    END IF;
END;
$$
LANGUAGE plpgsql;

CREATE TABLE IF NOT EXISTS notification (
    id         UUID                NOT NULL DEFAULT uuidv7(),

    target     NOTIFICATION_TARGET NOT NULL,
    url        VARCHAR             NOT NULL,

    name       VARCHAR             NOT NULL,

    owner      INTEGER             NOT NULL,

    created_at TIMESTAMPTZ         NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ         NOT NULL DEFAULT NOW(),

    PRIMARY KEY (id)
);
