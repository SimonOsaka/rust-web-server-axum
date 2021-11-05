-- Add up migration script here
ALTER TABLE my_adventures ADD COLUMN IF NOT EXISTS title_crypto varchar(50) NOT NULL DEFAULT '';

COMMENT ON COLUMN my_adventures.title_crypto IS 'crypto for unique title';