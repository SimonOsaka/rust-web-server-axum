-- Add up migration script here
CREATE TABLE "my_favorites" (
  "id" serial8 NOT NULL,
  "user_id" int8 NOT NULL DEFAULT 0,
  "adventure_id" int8 NOT NULL DEFAULT 0,
  "is_deleted" int2 NOT NULL DEFAULT 0,
  "created_at" timestamp(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT "my_favorites_pk" PRIMARY KEY ("id")
)
;