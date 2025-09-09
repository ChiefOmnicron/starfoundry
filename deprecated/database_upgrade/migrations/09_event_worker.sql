DO
$$
BEGIN
    IF NOT EXISTS (
        SELECT *
        FROM pg_type typ
        INNER JOIN pg_namespace nsp ON nsp.oid = typ.typnamespace
        WHERE nsp.nspname = current_schema()
        AND typ.typname = 'event_worker_task'
    ) THEN
        CREATE TYPE EVENT_WORKER_TASK AS ENUM (
            -- check if all asset tasks are in the queue
            'ASSET_CHECK',
            -- character blueprints
            'ASSET_CHARACTER_BLUEPRINTS',
            -- corporation blueprints
            'ASSET_CORPORATION_BLUEPRINTS',

            -- checks if all cleanup related events are in the queue
            'CLEANUP_CHECK',
            -- compresses the industry index
            'CLEANUP_APPRAISALS',
            -- compresses the industry index
            'CLEANUP_INDUSTRY_INDEX',
            -- cleanup event queue
            'CLEANUP_SELF',

            -- checks if all industry related events are in the queue
            -- corporation_jobs
            -- character_jobs
            'INDUSTRY_CHECK',
            -- pulls the indy jobs for a specific character and runs the job detection on it
            'INDUSTRY_JOBS_CHARACTER',
            -- pulls the indy jobs for a specific coproration nd runs the job detection on it
            'INDUSTRY_JOBS_CORPORATION',
            -- fetches the current industry index
            'INDUSTRY_INDEX',

            -- checks if all market related events are in the queue
            'MARKET_CHECK',
            -- fetches the latest NPC orders
            'MARKET_LATEST_NPC',
            -- fetches the latest player orders
            'MARKET_LATEST_PLAYER',
            -- fetches the latest market prices
            'MARKET_PRICES',

            -- checks if all sde tasks are in the queue
            -- sde_download
            'SDE_CHECK',
            -- downloads and imports the latest sde
            'SDE_DOWNLOAD',

            -- checks if all stock related events are in the queue
            -- blueprint_stock
            'STOCK_CHECK',
            -- checks the bpc stock and warns if necessary
            'STOCK_BLUEPRINT'
        );
    END IF;
END;
$$
LANGUAGE plpgsql;

DO
$$
BEGIN
    IF NOT EXISTS (
        SELECT *
        FROM pg_type typ
        INNER JOIN pg_namespace nsp ON nsp.oid = typ.typnamespace
        WHERE nsp.nspname = current_schema()
        AND typ.typname = 'event_task_status'
    ) THEN
        CREATE TYPE EVENT_TASK_STATUS AS ENUM (
            -- either the task is not ready yet, or a worker hasn't grabed it yet
            'WAITING',
            -- a worker grabed the task and is working on it
            'IN_PROGRESS',
            -- the task successfully finished
            'DONE',
            -- there was an error while task execution
            'ERROR'
        );
    END IF;
END;
$$
LANGUAGE plpgsql;

CREATE TABLE IF NOT EXISTS event_queue (
    id              UUID              NOT NULL DEFAULT gen_random_uuid(),
    -- when the job was put in the queue
    queued_at       TIMESTAMP         NOT NULL DEFAULT NOW(),
    -- time when the entry can be processed
    process_after   TIMESTAMP         NOT NULL DEFAULT NOW(),
    -- current status of the task
    status          EVENT_TASK_STATUS NOT NULL DEFAULT 'WAITING',
    -- task the worker should do when receiving the task
    task            EVENT_WORKER_TASK NOT NULL,
    -- id of the worker that is working on the task
    worker_id       UUID,
    -- time when a worker picked it up
    started_at      TIMESTAMP,
    -- time when the worker decides that it is done
    finished_at     TIMESTAMP,
    -- in case of an error, this will contain the infomration
    error           VARCHAR,
    -- additional data for example a character or corporation id
    additional_data JSONB,
    -- logs during that occured during execution
    logs            VARCHAR,

    PRIMARY KEY (id)
);
CREATE INDEX IF NOT EXISTS event_queue_event ON event_queue (task);

CREATE TABLE IF NOT EXISTS event_worker (
    id             UUID      NOT NULL DEFAULT gen_random_uuid(),
    last_seen      TIMESTAMP NOT NULL DEFAULT NOW(),
    active_since   TIMESTAMP NOT NULL DEFAULT NOW(),

    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS asset_blueprint (
    id                  UUID    NOT NULL DEFAULT gen_random_uuid(),
    -- is either a character_id or a corporation_id
    owner_id            INTEGER NOT NULL,

    type_id             INTEGER NOT NULL,
    quantity            INTEGER NOT NULL,
    runs                INTEGER NOT NULL,
    material_efficiency INTEGER NOT NULL,
    time_efficiency     INTEGER NOT NULL,

    PRIMARY KEY(id)
);
CREATE INDEX IF NOT EXISTS asset_blueprint_owner ON asset_blueprint (owner_id);
