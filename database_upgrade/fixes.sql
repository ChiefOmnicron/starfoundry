DELETE FROM _sqlx_migrations;

-- updates the updated_at field automatically
CREATE OR REPLACE FUNCTION trigger_set_updated_at()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

ALTER TABLE structure_groups RENAME TO structure_group;
ALTER TABLE structure_group ADD COLUMN created_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
ALTER TABLE structure_group ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
CREATE TRIGGER set_timestamp_structure_group BEFORE UPDATE ON structure_group FOR EACH ROW EXECUTE PROCEDURE trigger_set_updated_at();

ALTER TABLE credentials RENAME TO credential;
ALTER TABLE credential RENAME COLUMN created TO created_at;
ALTER TABLE credential ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
CREATE TRIGGER set_timestamp_credential BEFORE UPDATE ON credential FOR EACH ROW EXECUTE PROCEDURE trigger_set_updated_at();

ALTER TABLE characters RENAME TO character;
ALTER TABLE character ADD COLUMN created_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
ALTER TABLE character ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
CREATE TRIGGER set_timestamp_character BEFORE UPDATE ON character FOR EACH ROW EXECUTE PROCEDURE trigger_set_updated_at();

ALTER TABLE projects RENAME TO project;
ALTER TABLE project RENAME COLUMN notes TO note;
ALTER TABLE project ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
ALTER TABLE project ALTER COLUMN name TYPE VARCHAR(128);
ALTER TABLE project ALTER COLUMN orderer TYPE VARCHAR(128);
ALTER TABLE project ALTER COLUMN note TYPE VARCHAR(2048);
CREATE TRIGGER set_timestamp_project BEFORE UPDATE ON project FOR EACH ROW EXECUTE PROCEDURE trigger_set_updated_at();

ALTER TABLE project_products RENAME TO project_product;
ALTER TABLE project_product ADD COLUMN created_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
ALTER TABLE project_product ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
CREATE TRIGGER set_timestamp_project_product BEFORE UPDATE ON project_product FOR EACH ROW EXECUTE PROCEDURE trigger_set_updated_at();

ALTER TABLE project_jobs RENAME TO project_job;
ALTER TABLE project_job ADD COLUMN created_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
ALTER TABLE project_job ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
CREATE TRIGGER set_timestamp_project_job BEFORE UPDATE ON project_job FOR EACH ROW EXECUTE PROCEDURE trigger_set_updated_at();

ALTER TABLE project_market ADD COLUMN created_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
ALTER TABLE project_market ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
ALTER TABLE project_market ALTER COLUMN source TYPE VARCHAR(128);
CREATE TRIGGER set_timestamp_project_market BEFORE UPDATE ON project_market FOR EACH ROW EXECUTE PROCEDURE trigger_set_updated_at();

ALTER TABLE project_misc ADD COLUMN created_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
ALTER TABLE project_misc ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
ALTER TABLE project_misc ALTER COLUMN description TYPE VARCHAR(128);
CREATE TRIGGER set_timestamp_project_misc BEFORE UPDATE ON project_misc FOR EACH ROW EXECUTE PROCEDURE trigger_set_updated_at();

ALTER TABLE project_excess ADD COLUMN created_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
ALTER TABLE project_excess ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
CREATE TRIGGER set_timestamp_project_excess BEFORE UPDATE ON project_excess FOR EACH ROW EXECUTE PROCEDURE trigger_set_updated_at();

ALTER TABLE project_stocks RENAME TO project_stock;
ALTER TABLE project_stock ADD COLUMN created_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
ALTER TABLE project_stock ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
CREATE TRIGGER set_timestamp_project_stock BEFORE UPDATE ON project_stock FOR EACH ROW EXECUTE PROCEDURE trigger_set_updated_at();

ALTER TABLE project_market_structures RENAME TO project_market_structure;
ALTER TABLE project_market_structure ADD COLUMN created_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
ALTER TABLE project_market_structure ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
CREATE TRIGGER set_timestamp_project_market_structure BEFORE UPDATE ON project_market_structure FOR EACH ROW EXECUTE PROCEDURE trigger_set_updated_at();

ALTER TABLE project_blacklist ADD COLUMN created_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
ALTER TABLE project_blacklist ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
CREATE TRIGGER set_timestamp_project_blacklist BEFORE UPDATE ON project_blacklist FOR EACH ROW EXECUTE PROCEDURE trigger_set_updated_at();

ALTER TABLE project_groups RENAME TO project_group;
ALTER TABLE project_group ADD COLUMN created_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
ALTER TABLE project_group ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
ALTER TABLE project_group ALTER COLUMN name TYPE VARCHAR(128);
ALTER TABLE project_group ALTER COLUMN description TYPE VARCHAR(2048);
ALTER TABLE project_group DROP CONSTRAINT project_groups_owner_fkey;
CREATE TRIGGER set_timestamp_project_group BEFORE UPDATE ON project_group FOR EACH ROW EXECUTE PROCEDURE trigger_set_updated_at();

ALTER TABLE project_group_members RENAME TO project_group_member;
ALTER TABLE project_group_member ADD COLUMN created_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
ALTER TABLE project_group_member ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
CREATE TRIGGER set_timestamp_project_group_member BEFORE UPDATE ON project_group_member FOR EACH ROW EXECUTE PROCEDURE trigger_set_updated_at();

ALTER TABLE project_group_default_markets RENAME TO project_group_default_market;
ALTER TABLE project_group_default_market ADD COLUMN created_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
ALTER TABLE project_group_default_market ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
CREATE TRIGGER set_timestamp_project_group_default_market BEFORE UPDATE ON project_group_default_market FOR EACH ROW EXECUTE PROCEDURE trigger_set_updated_at();

ALTER TABLE project_group_default_blacklist ADD COLUMN created_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
ALTER TABLE project_group_default_blacklist ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
CREATE TRIGGER set_timestamp_project_group_default_blacklist BEFORE UPDATE ON project_group_default_blacklist FOR EACH ROW EXECUTE PROCEDURE trigger_set_updated_at();

ALTER TABLE project_job_assignments RENAME TO project_job_assignment;
ALTER TABLE project_job_assignment ADD COLUMN created_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
ALTER TABLE project_job_assignment ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
CREATE TRIGGER set_timestamp_project_job_assignment BEFORE UPDATE ON project_job_assignment FOR EACH ROW EXECUTE PROCEDURE trigger_set_updated_at();

ALTER TABLE stock_blueprints RENAME TO stock_blueprint;
ALTER TABLE stock_blueprint ADD COLUMN created_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
ALTER TABLE stock_blueprint ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
ALTER TABLE stock_blueprint ALTER COLUMN name TYPE VARCHAR(128);
ALTER TABLE stock_blueprint ALTER COLUMN description TYPE VARCHAR(2048);
CREATE TRIGGER set_timestamp_stock_blueprint BEFORE UPDATE ON stock_blueprint FOR EACH ROW EXECUTE PROCEDURE trigger_set_updated_at();

ALTER TABLE stock_blueprint_thresholds RENAME TO stock_blueprint_threshold;
ALTER TABLE stock_blueprint_threshold ADD COLUMN created_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
ALTER TABLE stock_blueprint_threshold ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
CREATE TRIGGER set_timestamp_stock_blueprint_threshold BEFORE UPDATE ON stock_blueprint_threshold FOR EACH ROW EXECUTE PROCEDURE trigger_set_updated_at();

ALTER TABLE market_prices RENAME TO market_price;
ALTER TABLE market_orders_latest RENAME TO market_order_latest;
CREATE INDEX IF NOT EXISTS market_order_latest_typeid ON market_order_latest(type_id);
CREATE UNIQUE INDEX IF NOT EXISTS market_order_latest_orderid ON market_order_latest(order_id);

ALTER TABLE appraisals RENAME TO appraisal;
CREATE UNIQUE INDEX IF NOT EXISTS appraisal_code ON appraisal(code);
ALTER TABLE appraisal_items RENAME TO appraisal_item;
CREATE INDEX IF NOT EXISTS appraisal_item_appraisalid ON appraisal_item(appraisal_id);
CREATE INDEX IF NOT EXISTS appraisal_invalid_appraisalid ON appraisal_invalid(appraisal_id);
ALTER TABLE appraisal_market_prices RENAME TO appraisal_market_price;

ALTER TABLE structures ADD COLUMN created_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
ALTER TABLE structures ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
ALTER TABLE structures ALTER COLUMN name TYPE VARCHAR(128);
CREATE TRIGGER set_timestamp_structure BEFORE UPDATE ON structures FOR EACH ROW EXECUTE PROCEDURE trigger_set_updated_at();
ALTER TABLE structures RENAME TO structure;
CREATE UNIQUE INDEX IF NOT EXISTS structure_id ON structure(id);

ALTER TABLE structure_project_group ADD COLUMN created_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
ALTER TABLE structure_project_group ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
CREATE TRIGGER set_timestamp_structure_project_group BEFORE UPDATE ON structure_project_group FOR EACH ROW EXECUTE PROCEDURE trigger_set_updated_at();
ALTER TABLE structure_project_group DROP CONSTRAINT structure_project_group_project_group_id_fkey;
ALTER TABLE structure_project_group DROP CONSTRAINT structure_project_group_structure_id_fkey;
ALTER TABLE structure_project_group ADD CONSTRAINT project_group_id_fkey FOREIGN KEY (project_group_id) REFERENCES project_group (id) ON DELETE NO ACTION;
ALTER TABLE structure_project_group ADD CONSTRAINT structure_id_fkey FOREIGN KEY (structure_id) REFERENCES structure (id) ON DELETE NO ACTION;

ALTER TABLE structure_dynamic_groups RENAME TO structure_dynamic_group;
ALTER TABLE structure_dynamic_group ADD COLUMN created_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
ALTER TABLE structure_dynamic_group ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
CREATE TRIGGER set_timestamp_structure_dynamic_group BEFORE UPDATE ON structure_dynamic_group FOR EACH ROW EXECUTE PROCEDURE trigger_set_updated_at();
ALTER TABLE structure_dynamic_group DROP CONSTRAINT structure_dynamic_groups_owner_fkey;
ALTER TABLE structure_dynamic_group ADD CONSTRAINT structure_dynamic_groups_owner_fkey FOREIGN KEY (owner) REFERENCES character (character_id) ON DELETE NO ACTION;

ALTER TABLE structure_rigs RENAME TO structure_rig;

ALTER TABLE industry_jobs RENAME TO industry_job;

ALTER TABLE notifications RENAME TO notification;
ALTER TABLE notification ADD COLUMN created_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
ALTER TABLE notification ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
CREATE TRIGGER set_timestamp_notification BEFORE UPDATE ON notification FOR EACH ROW EXECUTE PROCEDURE trigger_set_updated_at();

ALTER TABLE items RENAME TO item;
ALTER TABLE systems RENAME TO system;
ALTER TABLE blueprint_dependencies RENAME TO blueprint_dependency;
DROP TABLE IF EXISTS system_names;

ALTER TABLE event_workers RENAME TO event_worker;
ALTER TABLE job_detection_logs RENAME TO job_detection_log;
ALTER TABLE asset_blueprints RENAME TO asset_blueprint;
