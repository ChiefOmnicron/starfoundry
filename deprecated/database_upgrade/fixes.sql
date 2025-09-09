ALTER TABLE structure DROP CONSTRAINT structures_owner_fkey;
ALTER TABLE structure_group DROP CONSTRAINT structure_groups_owner_fkey;
ALTER TABLE structure_dynamic_group DROP CONSTRAINT structure_dynamic_groups_owner_fkey;

DROP VIEW rci_structures_grouped;
DROP VIEW rci_structures_total;
DROP TABLE wallet_character;
DROP TABLE wallet_corporation;
DROP TABLE eve_credential;
DROP TABLE character;
DROP TABLE credential;
DROP TABLE starfoundry_market_doctrine_ships;
DROP TABLE starfoundry_market_doctrines;
DROP TABLE starfoundry_market_fit_modules;
DROP TABLE starfoundry_market_fits;
DROP TABLE starfoundry_market_modules;
