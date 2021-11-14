-- Add up migration script here
ALTER TABLE my_adventures ADD COLUMN IF NOT EXISTS fav_count BIGINT NOT NULL DEFAULT 0;

COMMENT ON COLUMN my_adventures.fav_count IS 'my_favorites count';