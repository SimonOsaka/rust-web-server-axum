-- Add up migration script here
CREATE TABLE "my_adventures" (
  "id" serial8 NOT NULL,
  "title" varchar(40) NOT NULL DEFAULT '',
  "created_at" timestamp(0) NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "is_deleted" int2 NOT NULL DEFAULT 0,
  "image_url" varchar(255) NOT NULL DEFAULT '',
  "item_type" int2 NOT NULL DEFAULT 1,
  "link" varchar(255) NOT NULL DEFAULT '',
  "source" int2 NOT NULL DEFAULT 0,
  "journey_destiny" varchar(12) NOT NULL DEFAULT '',
  "script_content" varchar(140) NOT NULL DEFAULT '',
  "play_list" varchar(16) NOT NULL DEFAULT '',
  "douban_id" int8 NOT NULL DEFAULT 0,
  "douban_rank" int2 NOT NULL DEFAULT 0,
  "address" varchar(100) NOT NULL DEFAULT '',
  "shop_name" varchar(20) NOT NULL DEFAULT '',
  "province" varchar(7) NOT NULL DEFAULT '',
  "city" varchar(10) NOT NULL DEFAULT '',
  "district" varchar(10) NOT NULL DEFAULT '',
  CONSTRAINT "my_adventures_pkey" PRIMARY KEY ("id")
)
;