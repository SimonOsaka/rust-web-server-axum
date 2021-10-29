-- Add up migration script here
CREATE TABLE "my_adventures" (
  "id" serial8 NOT NULL,
  "title" varchar(40) COLLATE "pg_catalog"."default" NOT NULL DEFAULT ''::character varying,
  "created_at" timestamp(0) NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "is_deleted" int2 NOT NULL DEFAULT 0,
  "image_url" varchar(255) COLLATE "pg_catalog"."default" NOT NULL DEFAULT ''::character varying,
  "item_type" int2 NOT NULL DEFAULT 1,
  "link" varchar(255) COLLATE "pg_catalog"."default" NOT NULL DEFAULT ''::character varying,
  "source" int2 NOT NULL DEFAULT 0,
  "journey_destiny" varchar(12) COLLATE "pg_catalog"."default" NOT NULL DEFAULT ''::character varying,
  "script_content" varchar(140) COLLATE "pg_catalog"."default" NOT NULL DEFAULT ''::character varying,
  "play_list" varchar(16) COLLATE "pg_catalog"."default" NOT NULL DEFAULT ''::character varying,
  "douban_id" int8 NOT NULL DEFAULT 0,
  "douban_rank" int2 NOT NULL DEFAULT 0,
  "address" varchar(100) COLLATE "pg_catalog"."default" NOT NULL DEFAULT ''::character varying,
  "shop_name" varchar(20) COLLATE "pg_catalog"."default" NOT NULL DEFAULT ''::character varying,
  "province" varchar(7) COLLATE "pg_catalog"."default" NOT NULL DEFAULT ''::character varying,
  "city" varchar(10) COLLATE "pg_catalog"."default" NOT NULL DEFAULT ''::character varying,
  "district" varchar(10) COLLATE "pg_catalog"."default" NOT NULL DEFAULT ''::character varying,
  CONSTRAINT "my_adventures_pkey" PRIMARY KEY ("id")
)
;