-- Add up migration script here
CREATE TABLE "my_users" (
  "id" serial8 NOT NULL,
  "username" varchar(20) NOT NULL DEFAULT '',
  "password" varchar(50) NOT NULL DEFAULT '',
  "roles" varchar[] NOT NULL DEFAULT '{user}',
  "is_deleted" int2 NOT NULL DEFAULT 0,
  "created_at" timestamp(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT "my_user_pk" PRIMARY KEY ("id")
)
;