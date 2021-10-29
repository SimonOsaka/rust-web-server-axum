-- Add up migration script here
CREATE TABLE "my_users" (
  "id" serial8 NOT NULL,
  "username" varchar(20) COLLATE "pg_catalog"."default" NOT NULL DEFAULT ''::character varying,
  "password" varchar(50) COLLATE "pg_catalog"."default" NOT NULL DEFAULT ''::character varying,
  "roles" varchar[] COLLATE "pg_catalog"."default" NOT NULL DEFAULT '{user}'::character varying[],
  "is_deleted" int2 NOT NULL DEFAULT 0,
  "created_at" timestamp(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT "my_user_pk" PRIMARY KEY ("id")
)
;