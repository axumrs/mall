CREATE TYPE "user_status" AS ENUM('Pending', 'Actived', 'Freezed');

CREATE TABLE IF NOT EXISTS "users" (
    "id" CHAR(20) PRIMARY KEY,
    "email" VARCHAR(255) NOT NULL,
    "password" VARCHAR(255) NOT NULL,
    "nickname" VARCHAR(50) NOT NULL,
    "status" user_status NOT NULL DEFAULT 'Pending',
    "dateline" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "is_del" BOOLEAN NOT NULL DEFAULT FALSE,
    UNIQUE("email")
);

CREATE TABLE IF NOT EXISTS "brands" (
    "id" CHAR(20) PRIMARY KEY,
    "name" VARCHAR(100) NOT NULL,
    "logo" VARCHAR(255) NOT NULL,
    "is_del" BOOLEAN NOT NULL DEFAULT FALSE,
    "dateline" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE("name")
);