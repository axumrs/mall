CREATE TYPE "user_status" AS ENUM('Pending', 'Actived', 'Freezed');

CREATE TABLE IF NOT EXISTS "users" ( -- 用户
    "id" CHAR(20) PRIMARY KEY,
    "email" VARCHAR(255) NOT NULL, -- 邮箱
    "password" VARCHAR(255) NOT NULL, -- 密码
    "nickname" VARCHAR(50) NOT NULL, -- 昵称
    "status" user_status NOT NULL DEFAULT 'Pending', -- 状态
    "dateline" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "is_del" BOOLEAN NOT NULL DEFAULT FALSE,
    UNIQUE("email")
);

CREATE TABLE IF NOT EXISTS "brands" ( -- 品牌
    "id" CHAR(20) PRIMARY KEY,
    "name" VARCHAR(100) NOT NULL, -- 名称
    "logo" VARCHAR(255) NOT NULL, -- LOGO
    "is_del" BOOLEAN NOT NULL DEFAULT FALSE,
    "dateline" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE("name")
);

CREATE TYPE "category_level" AS ENUM('Unspecified', 'Level1', 'Level2', 'Level3');

CREATE TABLE IF NOT EXISTS "categoies" ( -- 商品分类
    "id" CHAR(20) PRIMARY KEY,
    "name" VARCHAR(100) NOT NULL, -- 名称
    "parent" CHAR(20) NOT NULL DEFAULT '', -- 父级分类
    "path" VARCHAR NOT NULL DEFAULT '', -- 路径
    "level" category_level NOT NULL DEFAULT 'Unspecified', -- 级别
    "dateline" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "is_del" BOOLEAN NOT NULL DEFAULT FALSE,
    UNIQUE("name", "parent")
);

CREATE TABLE IF NOT EXISTS "category_brands" ( -- 商品分类与品牌
    "brand_id" CHAR(20) NOT NULL, -- 品牌ID
    "category_id" CHAR(20) NOT NULL, -- 分类ID
    PRIMARY KEY("brand_id", "category_id")
);
