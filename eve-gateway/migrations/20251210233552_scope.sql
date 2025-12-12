ALTER TABLE eve_credential ADD COLUMN IF NOT EXISTS scopes VARCHAR[] DEFAULT ARRAY[]::VARCHAR[];
ALTER TABLE eve_credential ADD COLUMN IF NOT EXISTS domain VARCHAR NOT NULL;

ALTER TABLE eve_credential DROP CONSTRAINT IF EXISTS eve_credential_pkey;
ALTER TABLE eve_credential ADD CONSTRAINT eve_credential_pkey PRIMARY KEY (character_id, domain);

CREATE INDEX IF NOT EXISTS eve_credential_characterid ON eve_credential(character_id);
