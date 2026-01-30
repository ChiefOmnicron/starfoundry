DO
$$
BEGIN
    IF NOT EXISTS (
        SELECT *
        FROM pg_type typ
        INNER JOIN pg_namespace nsp ON nsp.oid = typ.typnamespace
        WHERE nsp.nspname = current_schema()
        AND typ.typname = 'worker_task_status'
    ) THEN
        CREATE TYPE WORKER_TASK_STATUS AS ENUM (
            -- either the task is not ready yet, or a worker hasn't grabed it yet
            'WAITING',
            -- a worker grabed the task and is working on it
            'IN_PROGRESS',
            -- the task successfully finished
            'DONE',
            -- there was an error while task execution
            'ERROR',
            -- the task went into a timeout
            'TIMEOUT'
        );
    END IF;
END;
$$
LANGUAGE plpgsql;

CREATE TABLE IF NOT EXISTS worker_queue (
    id              UUID                NOT NULL DEFAULT uuidv7(),
    -- when the job was put in the queue
    queued_at       TIMESTAMP           NOT NULL DEFAULT NOW(),
    -- time when the entry can be processed
    process_after   TIMESTAMP           NOT NULL DEFAULT NOW(),
    -- current status of the task
    status          WORKER_TASK_STATUS  NOT NULL DEFAULT 'WAITING',
    -- task the worker should do when receiving the task
    task            VARCHAR             NOT NULL,
    -- id of the worker that is working on the task
    -- defines if a task originated from a different task
    -- subtasks have a lower priority
    is_subtask      BOOLEAN             NOT NULL DEFAULT false,

    -- id of the worker that is working on the task
    worker_id       UUID,
    -- time when a worker picked it up
    started_at      TIMESTAMP,
    -- time when the worker decides that it is done
    finished_at     TIMESTAMP,
    -- in case of an error, this will contain the information
    error           VARCHAR,
    -- additional data for example a character or corporation id
    additional_data JSONB,
    -- logs during that occurred during execution
    logs            VARCHAR,

    PRIMARY KEY (id)
);
CREATE INDEX IF NOT EXISTS worker_queue_event ON worker_queue (task);

CREATE TABLE IF NOT EXISTS worker_registry (
    id             UUID      NOT NULL DEFAULT uuidv7(),
    last_seen      TIMESTAMP NOT NULL DEFAULT NOW(),
    active_since   TIMESTAMP NOT NULL DEFAULT NOW(),

    PRIMARY KEY (id)
);
