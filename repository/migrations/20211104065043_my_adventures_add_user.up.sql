-- Add up migration script here
ALTER TABLE my_adventures ADD COLUMN IF NOT EXISTS user_id BIGINT NOT NULL DEFAULT 0;

COMMENT ON COLUMN my_adventures.user_id IS 'my_users id';