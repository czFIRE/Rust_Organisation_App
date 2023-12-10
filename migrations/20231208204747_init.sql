-- Enums

CREATE TYPE "Gender"              AS ENUM ('male', 'female');
CREATE TYPE "Role"                AS ENUM ('user', 'admin');
CREATE TYPE "Association"         AS ENUM ('sponsor', 'organization', 'media');
CREATE TYPE "TaskPriority"        AS ENUM ('low', 'medium', 'high');
CREATE TYPE "AcceptanceStatus"    AS ENUM ('pending', 'accepted', 'rejected');
CREATE TYPE "EmployeeLevel"       AS ENUM ('basic', 'organizer', 'manager');
CREATE TYPE "EmployeeContract"    AS ENUM ('full-time', 'part-time', 'temporary');
CREATE TYPE "EmployeeStatus"      AS ENUM ('available', 'unavailable');
CREATE TYPE "EventRole"           AS ENUM ('worker', 'staff');

-- Tables



CREATE TABLE IF NOT EXISTS "timesheet"
(
    id          SERIAL PRIMARY KEY,
    ---------------------------------------------
    created_at  TIMESTAMP NOT NULL DEFAULT now(),
    edited_at   TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at  TIMESTAMP
);


CREATE TABLE IF NOT EXISTS "work_day"
(
    created_at  TIMESTAMP NOT NULL DEFAULT now(),
    edited_at   TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at  TIMESTAMP
);


CREATE TABLE IF NOT EXISTS "event"
(
    id          SERIAL PRIMARY KEY,
    ---------------------------------------------
    created_at  TIMESTAMP NOT NULL DEFAULT now(),
    edited_at   TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at  TIMESTAMP
);


CREATE TABLE IF NOT EXISTS "event_staff"
(
    created_at  TIMESTAMP NOT NULL DEFAULT now(),
    edited_at   TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at  TIMESTAMP
);


CREATE TABLE IF NOT EXISTS "task"
(
    id          SERIAL PRIMARY KEY,
    ---------------------------------------------
    created_at  TIMESTAMP NOT NULL DEFAULT now(),
    edited_at   TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at  TIMESTAMP
);


CREATE TABLE IF NOT EXISTS "taskasignee"
(
    created_at  TIMESTAMP NOT NULL DEFAULT now(),
    edited_at   TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at  TIMESTAMP
);


CREATE TABLE IF NOT EXISTS "comment"
(
    id          SERIAL PRIMARY KEY,
    ---------------------------------------------
    created_at  TIMESTAMP NOT NULL DEFAULT now(),
    edited_at   TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at  TIMESTAMP
);


CREATE TABLE IF NOT EXISTS "user"
(
    id          SERIAL PRIMARY KEY,
    ---------------------------------------------
    created_at  TIMESTAMP NOT NULL DEFAULT now(),
    edited_at   TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at  TIMESTAMP
);


CREATE TABLE IF NOT EXISTS "company"
(
    id          SERIAL PRIMARY KEY,
    ---------------------------------------------
    created_at  TIMESTAMP NOT NULL DEFAULT now(),
    edited_at   TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at  TIMESTAMP
);


CREATE TABLE IF NOT EXISTS "associated_company"
(
    created_at  TIMESTAMP NOT NULL DEFAULT now(),
    edited_at   TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at  TIMESTAMP
);


CREATE TABLE IF NOT EXISTS "company_address"
(
    -- TODO
);


CREATE TABLE IF NOT EXISTS "employment"
(
    created_at  TIMESTAMP NOT NULL DEFAULT now(),
    edited_at   TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at  TIMESTAMP
);
