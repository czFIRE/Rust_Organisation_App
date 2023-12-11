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

-- Domains

CREATE DOMAIN ufloat AS float
    CHECK(VALUE >= 0.0);
CREATE DOMAIN hours_per_month_float AS ufloat
    -- check value is <= than max hours per month (24.0 * 31.0)
    CHECK(VALUE <= 744.0);
CREATE DOMAIN hours_per_day_float AS ufloat
    -- check value is <= than max hours per day
    CHECK(VALUE <= 24.0);

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

CREATE TABLE IF NOT EXISTS "user"
(
    user_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    ---------------------------------------------
    avatar_id uuid REFERENCES "file"(file_id),
    ---------------------------------------------
    name varchar(255) NOT NULL,
    user_level "UserLevel" NOT NULL DEFAULT 'user',
    user_status "UserStatus" NOT NULL DEFAULT 'ok',
    email varchar(45) NOT NULL,
    date_of_birth date NOT NULL,
    sex "UserSex" NOT NULL,
    created_at timestamp NOT NULL DEFAULT now(),
    edited_at timestamp NOT NULL DEFAULT now(),
    deleted_at timestamp,

    CONSTRAINT check_user_created_at_lte_edited_at
        CHECK (edited_at >= created_at)
);


CREATE TABLE IF NOT EXISTS "event" (
    event_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    ---------------------------------------------
    avatar_id uuid REFERENCES "file"(file_id),
    ---------------------------------------------
    name varchar(255) NOT NULL UNIQUE,
    description text NOT NULL DEFAULT '',
    website varchar(4096) NOT NULL DEFAULT '',
    accepts_staff bool NOT NULL DEFAULT true,
    work_start date NOT NULL,
    work_end date NOT NULL,
    created_at timestamp NOT NULL DEFAULT now(),
    edited_at timestamp NOT NULL DEFAULT now(),
    deleted_at timestamp,

    CONSTRAINT check_event_name_len
        CHECK (char_length(name) >= 1),
    CONSTRAINT check_event_work_start_lte_work_end
        CHECK (work_start >= work_end),
    CONSTRAINT check_event_created_at_lte_edited_at
        CHECK (edited_at >= created_at)
);


CREATE TABLE IF NOT EXISTS "company" (
    company_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    ---------------------------------------------
    avatar_id uuid REFERENCES "file"(file_id),
    ---------------------------------------------
    name varchar(255) NOT NULL UNIQUE,
    description text NOT NULL DEFAULT '',
    website varchar(4096) NOT NULL DEFAULT '',
    crn varchar(16) NOT NULL,
    vatin varchar(18) NOT NULL,
    created_at timestamp NOT NULL DEFAULT now(),
    edited_at timestamp NOT NULL DEFAULT now(),
    deleted_at timestamp,

    CONSTRAINT check_company_name_len
        CHECK (char_length(name) >= 1),
    CONSTRAINT check_company_created_at_lte_edited_at
        CHECK (edited_at >= created_at)
);


CREATE TABLE IF NOT EXISTS "company_address" (
    company_id uuid PRIMARY KEY REFERENCES "company"(company_id)
        DEFAULT gen_random_uuid(),
    ---------------------------------------------
    country varchar(255) NOT NULL,
    region varchar(255) NOT NULL,
    city varchar(255) NOT NULL,
    street varchar(255) NOT NULL,
    address_number varchar(255) NOT NULL,
    postal_code varchar(255) NOT NULL,

    CONSTRAINT check_company_address_country_len
        CHECK (char_length(country) >= 1),
    CONSTRAINT check_company_address_region_len
        CHECK (char_length(region) >= 1),
    CONSTRAINT check_company_address_city_len
        CHECK (char_length(city) >= 1),
    CONSTRAINT check_company_address_street_len
        CHECK (char_length(street) >= 1),
    CONSTRAINT check_company_address_address_number_len
        CHECK (char_length(address_number) >= 1),
    CONSTRAINT check_company_address_postal_code_len
        CHECK (char_length(postal_code) >= 1)

);

CREATE TABLE IF NOT EXISTS "associated_company" (
    event_id uuid REFERENCES "event"(event_id),
    company_id uuid REFERENCES "company"(company_id),
    ---------------------------------------------
    association_type "AssociationType" NOT NULL,
    created_at timestamp NOT NULL DEFAULT now(),
    edited_at timestamp NOT NULL DEFAULT now(),
    deleted_at timestamp,

    PRIMARY KEY (event_id, company_id),
    CONSTRAINT check_associated_company_created_at_lte_edited_at
        CHECK (edited_at >= created_at)
);


CREATE TABLE IF NOT EXISTS "timesheet" (
    timesheet_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    ---------------------------------------------
    user_id uuid NOT NULL REFERENCES "user"(user_id),
    company_id uuid NOT NULL REFERENCES "company"(company_id),
    event_id uuid NOT NULL REFERENCES "event"(event_id),
    ---------------------------------------------
    start_date date NOT NULL,
    end_date date NOT NULL,
    worked_hours hours_per_month_float NOT NULL DEFAULT 0.0,
    is_editable boolean NOT NULL DEFAULT true,
    manager_note text NOT NULL DEFAULT '',
    created_at timestamp NOT NULL,
    edited_at timestamp NOT NULL,
    deleted_at timestamp,

    CONSTRAINT check_timesheet_start_date_lte_end_date
        CHECK (start_date >= end_date),
    CONSTRAINT check_timesheet_created_at_lte_edited_at
        CHECK (edited_at >= created_at)
);


CREATE TABLE IF NOT EXISTS "work_day"
(
    timesheet_id uuid REFERENCES "timesheet"(timesheet_id),
    work_date date,
    ---------------------------------------------
    worked_hours hours_per_day_float NOT NULL,
    commentary text NOT NULL DEFAULT '',
    is_editable boolean NOT NULL DEFAULT true,
    created_at timestamp NOT NULL DEFAULT now(),
    edited_at timestamp NOT NULL DEFAULT now(),
    deleted_at timestamp,

    PRIMARY KEY (timesheet_id, work_date),
    CONSTRAINT check_work_day_created_at_lte_edited_at
        CHECK (edited_at >= created_at)
);


CREATE TABLE IF NOT EXISTS "event_staff"
(
    user_id uuid REFERENCES "user"(user_id),
    event_id uuid REFERENCES "event"(event_id),
    ---------------------------------------------
    staff_level "StaffLevel" NOT NULL DEFAULT 'basic',
    acceptance_status "AcceptanceStatus" NOT NULL DEFAULT 'pending',
    -- todo: decided_by
    created_at  TIMESTAMP NOT NULL DEFAULT now(),
    edited_at   TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at  TIMESTAMP,

    PRIMARY KEY (user_id, event_id),
    CONSTRAINT check_event_stuff_created_at_lte_edited_at
        CHECK (edited_at >= created_at)
);


CREATE TABLE IF NOT EXISTS "task" (
    task_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    ---------------------------------------------
    event_id uuid NOT NULL REFERENCES "event"(event_id),
    ---------------------------------------------
    -- todo: creator_id
    title text NOT NULL,
    description text NOT NULL DEFAULT '',
    date_accomplished timestamp,
    priority "TaskPriority" NOT NULL,
    created_at timestamp NOT NULL,
    edited_at timestamp NOT NULL,
    deleted_at timestamp,

    CONSTRAINT check_event_title_len
        CHECK (char_length(title) >= 1),
    CONSTRAINT check_event_stuff_created_at_lte_edited_at
        CHECK (edited_at >= created_at)
);


CREATE TABLE IF NOT EXISTS "comment"
(
    comment_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    ---------------------------------------------
    event_id uuid REFERENCES "event"(event_id),
    task_id uuid REFERENCES "task"(task_id),
    author_id uuid NOT NULL REFERENCES "user"(user_id),
    content text NOT NULL,
    created_at timestamp NOT NULL,
    edited_at timestamp NOT NULL,
    deleted_at timestamp

    CONSTRAINT check_comment_single_relation_only
        CHECK (
            (CASE WHEN event_id IS NULL THEN 0 ELSE 1 END
             + CASE WHEN task_id IS NULL THEN 0 ELSE 1 END
            ) = 1),
    CONSTRAINT check_comment_created_at_lte_edited_at
        CHECK (edited_at >= created_at)
);


CREATE TABLE IF NOT EXISTS "employment"
(
    user_id uuid NOT NULL REFERENCES "user"(user_id),
    company_id uuid NOT NULL REFERENCES "company"(company_id),
    ---------------------------------------------
    -- todo: manager_id
    employment_type "EmploymentType" NOT NULL,
    hourly_rate float NOT NULL,
    employee_level "EmployeeLevel" NOT NULL,
    description text NOT NULL DEFAULT '',
    start_date date NOT NULL,
    end_date date NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    edited_at TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at TIMESTAMP,

    PRIMARY KEY (user_id, company_id),
    CONSTRAINT check_employment_start_date_lte_end_date
        CHECK (start_date >= end_date),
    CONSTRAINT check_employment_created_at_lte_edited_at
        CHECK (edited_at >= created_at)
);
