-- Add new entries
ALTER TYPE EVENT_WORKER_TASK RENAME TO EVENT_WORKER_TASK_;

CREATE TYPE EVENT_WORKER_TASK AS ENUM (
    -- check if all asset tasks are in the queue
    'ASSET_CHECK',
    -- pulls the blueprints the character posses
    'ASSET_CHARACTER_BLUEPRINTS',
    -- pulls the blueprints the corporation posses
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

ALTER TABLE event_queue ALTER COLUMN task TYPE EVENT_WORKER_TASK USING task::text::EVENT_WORKER_TASK;

DROP TYPE EVENT_WORKER_TASK_;
