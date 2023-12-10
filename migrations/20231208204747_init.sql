--
-- Notes:
--
-- + Constraints are named using `<constraint type>_<table_name>_...`
--   convention (thus e.g. `check_user_...`) as described at:
--   https://fossies.org/linux/gitlab-foss/doc/development/database/
--     constraint_naming_convention.md
--

-- Enums

CREATE TYPE "UserSex"             AS ENUM ('male', 'female', 'other');
CREATE TYPE "UserLevel"           AS ENUM ('user', 'admin');
CREATE TYPE "AssociationType"     AS ENUM ('sponsor', 'organizer', 'other');
CREATE TYPE "TaskPriority"        AS ENUM ('low', 'medium', 'high');
CREATE TYPE "AssignmentStatus"    AS ENUM ('pending', 'accepted', 'rejected');
CREATE TYPE "AcceptanceStatus"    AS ENUM ('pending', 'accepted', 'rejected');
CREATE TYPE "EmployeeLevel"       AS ENUM ('basic', 'manager', 'upper manager');
CREATE TYPE "StaffLevel"          AS ENUM ('basic', 'organizer');
CREATE TYPE "EmploymentType"      AS ENUM ('DPP', 'DPC', 'HPP');
CREATE TYPE "UserStatus"          AS ENUM ('ok', 'sick', 'vacation');

-- Tables

CREATE TABLE IF NOT EXISTS "file" (
    file_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    ---------------------------------------------
    extension varchar(20) NOT NULL DEFAULT '',
    params varchar(60) NOT NULL DEFAULT '',
    created_at timestamp NOT NULL DEFAULT now(),
    edited_at timestamp NOT NULL DEFAULT now(),
    deleted_at timestamp,

    CONSTRAINT check_file_created_at_lte_edited_at
        CHECK (edited_at >= created_at)
);


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
