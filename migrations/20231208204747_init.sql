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

CREATE TABLE IF NOT EXISTS "Timesheet"
(
    id          SERIAL PRIMARY KEY,
    ---------------------------------------------
    created_at  TIMESTAMP NOT NULL DEFAULT now(),
    edited_at   TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at  TIMESTAMP
);


CREATE TABLE IF NOT EXISTS "WorkDay"
(
    created_at  TIMESTAMP NOT NULL DEFAULT now(),
    edited_at   TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at  TIMESTAMP
);


CREATE TABLE IF NOT EXISTS "Event"
(
    id          SERIAL PRIMARY KEY,
    ---------------------------------------------
    created_at  TIMESTAMP NOT NULL DEFAULT now(),
    edited_at   TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at  TIMESTAMP
);


CREATE TABLE IF NOT EXISTS "EventStaff"
(
    created_at  TIMESTAMP NOT NULL DEFAULT now(),
    edited_at   TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at  TIMESTAMP
);


CREATE TABLE IF NOT EXISTS "Task"
(
    id          SERIAL PRIMARY KEY,
    ---------------------------------------------
    created_at  TIMESTAMP NOT NULL DEFAULT now(),
    edited_at   TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at  TIMESTAMP
);


CREATE TABLE IF NOT EXISTS "TaskAsignee"
(
    created_at  TIMESTAMP NOT NULL DEFAULT now(),
    edited_at   TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at  TIMESTAMP
);


CREATE TABLE IF NOT EXISTS "Comment"
(
    id          SERIAL PRIMARY KEY,
    ---------------------------------------------
    created_at  TIMESTAMP NOT NULL DEFAULT now(),
    edited_at   TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at  TIMESTAMP
);


CREATE TABLE IF NOT EXISTS "User"
(
    id          SERIAL PRIMARY KEY,
    ---------------------------------------------
    created_at  TIMESTAMP NOT NULL DEFAULT now(),
    edited_at   TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at  TIMESTAMP
);


CREATE TABLE IF NOT EXISTS "Company"
(
    id          SERIAL PRIMARY KEY,
    ---------------------------------------------
    created_at  TIMESTAMP NOT NULL DEFAULT now(),
    edited_at   TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at  TIMESTAMP
);


CREATE TABLE IF NOT EXISTS "AssociatedCompany"
(
    created_at  TIMESTAMP NOT NULL DEFAULT now(),
    edited_at   TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at  TIMESTAMP
);


CREATE TABLE IF NOT EXISTS "CompanyAddress"
(
    -- TODO
);


CREATE TABLE IF NOT EXISTS "Employment"
(
    created_at  TIMESTAMP NOT NULL DEFAULT now(),
    edited_at   TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at  TIMESTAMP
);
