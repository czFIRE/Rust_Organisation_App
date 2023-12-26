-- Enums

CREATE TYPE gender                  AS ENUM ('male', 'female', 'other');
CREATE TYPE role                    AS ENUM ('user', 'admin');
CREATE TYPE status                  AS ENUM ('available', 'unavailable');
CREATE TYPE association             AS ENUM ('sponsor', 'organization', 'media', 'other');
CREATE TYPE task_priority           AS ENUM ('low', 'medium', 'high');
CREATE TYPE acceptance_status       AS ENUM ('pending', 'accepted', 'rejected');
CREATE TYPE employee_level          AS ENUM ('basic', 'manager', 'company_administrator');
CREATE TYPE employee_contract       AS ENUM ('DPP', 'DPC', 'HPP');
CREATE TYPE event_role              AS ENUM ('staff', 'organizer');


-- Constraints

CREATE DOMAIN ufloat AS float CHECK(VALUE >= 0.0);
-- max hours per month: 24.0 * 31.0 = 744.0
CREATE DOMAIN hours_per_month_float AS ufloat CHECK(VALUE <= 744.0);
CREATE DOMAIN hours_per_day_float AS ufloat CHECK(VALUE <= 24.0);


-- Tables

-- BASIC ENTTITIES

CREATE TABLE user_record
(
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    -------------------------------------------------------
    name        VARCHAR(255) NOT NULL,
    email       VARCHAR(255) NOT NULL UNIQUE,
    birth       DATE NOT NULL,
    avatar_url  VARCHAR(255),
    gender      gender NOT NULL,
    role        role NOT NULL,
    status      status NOT NULL,
    -------------------------------------------------------
    created_at  TIMESTAMP NOT NULL DEFAULT now(),
    edited_at   TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at  TIMESTAMP,
    -------------------------------------------------------
    CONSTRAINT check_user_created_at_lte_edited_at
        CHECK (edited_at >= created_at)
);


CREATE TABLE company
(
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    -------------------------------------------------------
    name        VARCHAR(255) NOT NULL,
    description TEXT,
    website     VARCHAR(255),
    crn         VARCHAR(16) NOT NULL UNIQUE,
    vatin       VARCHAR(18) NOT NULL UNIQUE,
    phone       VARCHAR(255) NOT NULL UNIQUE,
    email       VARCHAR(255) NOT NULL UNIQUE,
    avatar_url  VARCHAR(255),
    -------------------------------------------------------
    created_at  TIMESTAMP NOT NULL DEFAULT now(),
    edited_at   TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at  TIMESTAMP,
    -------------------------------------------------------
    CONSTRAINT check_company_name_len
        CHECK (char_length(name) >= 1),
    CONSTRAINT check_company_created_at_lte_edited_at
        CHECK (edited_at >= created_at)
);


CREATE TABLE address
(
    country     VARCHAR(255) NOT NULL,
    region      VARCHAR(255) NOT NULL,
    city        VARCHAR(255) NOT NULL,
    street      VARCHAR(255) NOT NULL,
    number      VARCHAR(255) NOT NULL,
    postal_code VARCHAR(255) NOT NULL,
    -------------------------------------------------------
    company_id  UUID NOT NULL,
    -------------------------------------------------------
    PRIMARY KEY (company_id),
    FOREIGN KEY (company_id) REFERENCES company (id),
    -------------------------------------------------------
    CONSTRAINT check_company_address_country_len
        CHECK (char_length(country) >= 1),
    CONSTRAINT check_company_address_region_len
        CHECK (char_length(region) >= 1),
    CONSTRAINT check_company_address_city_len
        CHECK (char_length(city) >= 1),
    CONSTRAINT check_company_address_street_len
        CHECK (char_length(street) >= 1),
    CONSTRAINT check_company_address_address_number_len
        CHECK (char_length(number) >= 1),
    CONSTRAINT check_company_address_postal_code_len
        CHECK (char_length(postal_code) >= 1)
);


CREATE TABLE employment
(
    hourly_wage FLOAT NOT NULL,
    start_date  DATE NOT NULL,
    end_date    DATE NOT NULL,
    description TEXT,
    type        employee_contract NOT NULL,
    level       employee_level NOT NULL,
    -------------------------------------------------------
    created_at  TIMESTAMP NOT NULL DEFAULT now(),
    edited_at   TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at  TIMESTAMP,
    -------------------------------------------------------
    user_id     UUID NOT NULL,
    company_id  UUID NOT NULL,
    manager_id  UUID,
    -------------------------------------------------------
    PRIMARY KEY (user_id, company_id),
    FOREIGN KEY (user_id) REFERENCES user_record (id),
    FOREIGN KEY (company_id) REFERENCES company (id),
    FOREIGN KEY (manager_id) REFERENCES user_record  (id),
    -------------------------------------------------------
    CONSTRAINT check_employment_start_date_lte_end_date
        CHECK (start_date >= end_date),
    CONSTRAINT check_employment_created_at_lte_edited_at
        CHECK (edited_at >= created_at)
);


-- EVENT ENTITIES

CREATE TABLE event
(
    id             UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    -------------------------------------------------------
    name           VARCHAR(255) NOT NULL,
    description    TEXT,
    website        VARCHAR(255),
    accepts_staff  BOOLEAN NOT NULL,
    start_date     DATE NOT NULL,
    end_date       DATE NOT NULL,
    avatar_url     VARCHAR(255),
    -------------------------------------------------------
    created_at     TIMESTAMP NOT NULL DEFAULT now(),
    edited_at      TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at     TIMESTAMP,
    -------------------------------------------------------
    CONSTRAINT check_event_name_len
        CHECK (char_length(name) >= 1),
    CONSTRAINT check_event_start_date_lte_end_date
        CHECK (start_date >= end_date),
    CONSTRAINT check_event_created_at_lte_edited_at
        CHECK (edited_at >= created_at)
);


CREATE TABLE associated_company
(
    type        association NOT NULL,
    -------------------------------------------------------
    created_at  TIMESTAMP NOT NULL DEFAULT now(),
    edited_at   TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at  TIMESTAMP,
    -------------------------------------------------------
    company_id  UUID NOT NULL,
    event_id    UUID NOT NULL,
    -------------------------------------------------------
    PRIMARY KEY (company_id, event_id),
    FOREIGN KEY (company_id) REFERENCES company (id),
    FOREIGN KEY (event_id) REFERENCES event (id),
    -------------------------------------------------------
    CONSTRAINT check_associated_company_created_at_lte_edited_at
        CHECK (edited_at >= created_at)
);


CREATE TABLE timesheet
(
    id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    -------------------------------------------------------
    start_date   DATE NOT NULL,
    end_date     DATE NOT NULL,
    total_hours  hours_per_month_float NOT NULL DEFAULT 0.0,
    is_editable  BOOLEAN NOT NULL,
    manager_note TEXT,
    -------------------------------------------------------
    created_at   TIMESTAMP NOT NULL DEFAULT now(),
    edited_at    TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at   TIMESTAMP,
    --------------------------------------------------------
    user_id      UUID NOT NULL,
    company_id   UUID NOT NULL,
    event_id     UUID NOT NULL,
    --------------------------------------------------------
    FOREIGN KEY  (user_id) REFERENCES user_record (id),
    FOREIGN KEY  (company_id) REFERENCES company (id),
    FOREIGN KEY  (event_id) REFERENCES event (id),
    --------------------------------------------------------
    CONSTRAINT check_timesheet_start_date_lte_end_date
        CHECK (start_date >= end_date),
    CONSTRAINT check_timesheet_created_at_lte_edited_at
        CHECK (edited_at >= created_at)
);


CREATE TABLE work_day
(   
    date         DATE NOT NULL,
    total_hours  hours_per_day_float NOT NULL DEFAULT 0.0,
    comment      TEXT,
    is_editable  BOOLEAN NOT NULL,
    --------------------------------------------------------
    created_at   TIMESTAMP NOT NULL DEFAULT now(),
    edited_at    TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at   TIMESTAMP,
    --------------------------------------------------------
    timesheet_id UUID NOT NULL,
    --------------------------------------------------------
    PRIMARY KEY  (timesheet_id, date),
    FOREIGN KEY  (timesheet_id) REFERENCES timesheet (id),
    --------------------------------------------------------
    CONSTRAINT check_work_day_created_at_lte_edited_at
        CHECK (edited_at >= created_at)
);


-- WORKFLOW ENTITIES

CREATE TABLE event_staff
(
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    -------------------------------------------------------
    role        event_role NOT NULL,
    status      acceptance_status NOT NULL,
    -------------------------------------------------------
    created_at  TIMESTAMP NOT NULL DEFAULT now(),
    edited_at   TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at  TIMESTAMP,
    -------------------------------------------------------
    user_id     UUID NOT NULL,
    company_id  UUID NOT NULL,
    decided_by  UUID NOT NULL,
    event_id    UUID NOT NULL,
    -------------------------------------------------------
    FOREIGN KEY (user_id) REFERENCES user_record (id),
    FOREIGN KEY (company_id) REFERENCES company (id),
    FOREIGN KEY (decided_by) REFERENCES event_staff (id),
    FOREIGN KEY (event_id) REFERENCES event (id),
    -------------------------------------------------------
    CONSTRAINT check_event_stuff_created_at_lte_edited_at
        CHECK (edited_at >= created_at)
);


CREATE TABLE task
(
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    -------------------------------------------------------
    title           VARCHAR(255) NOT NULL,
    description     TEXT,
    finished_at     TIMESTAMP,
    priority        task_priority NOT NULL,
    accepts_staff   BOOLEAN NOT NULL,
    -------------------------------------------------------
    created_at  TIMESTAMP NOT NULL DEFAULT now(),
    edited_at   TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at  TIMESTAMP,
    -------------------------------------------------------
    event_id    UUID NOT NULL,
    creator_id  UUID NOT NULL,
    -------------------------------------------------------
    FOREIGN KEY (event_id) REFERENCES event (id),
    FOREIGN KEY (creator_id) REFERENCES event_staff (id),
    -------------------------------------------------------
    CONSTRAINT check_event_title_len
        CHECK (char_length(title) >= 1),
    CONSTRAINT check_event_stuff_created_at_lte_edited_at
        CHECK (edited_at >= created_at)
);


CREATE TABLE assigned_staff
(
    status      acceptance_status NOT NULL,
    -------------------------------------------------------
    created_at  TIMESTAMP NOT NULL DEFAULT now(),
    edited_at   TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at  TIMESTAMP,
    -------------------------------------------------------
    decided_by  UUID NOT NULL,
    task_id     UUID NOT NULL,
    staff_id    UUID NOT NULL,
    -------------------------------------------------------
    PRIMARY KEY (staff_id, task_id),
    FOREIGN KEY (decided_by) REFERENCES event_staff (id),
    FOREIGN KEY (task_id) REFERENCES task (id),
    FOREIGN KEY (staff_id) REFERENCES event_staff (id),
    -------------------------------------------------------
    CONSTRAINT check_assigned_staff_created_at_lte_edited_at
        CHECK (edited_at >= created_at)
);


CREATE TABLE comment
(
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    -------------------------------------------------------
    content     TEXT NOT NULL,
    -------------------------------------------------------
    created_at  TIMESTAMP NOT NULL DEFAULT now(),
    edited_at   TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at  TIMESTAMP,
    -------------------------------------------------------
    event_id    UUID,
    author_id   UUID NOT NULL,
    task_id     UUID,
    -------------------------------------------------------
    FOREIGN KEY (event_id) REFERENCES event (id),
    FOREIGN KEY (author_id) REFERENCES user_record (id),
    FOREIGN KEY (task_id) REFERENCES task (id),
    -------------------------------------------------------
    CONSTRAINT check_comment_single_relation_only
        CHECK (
            (CASE WHEN event_id IS NULL THEN 0 ELSE 1 END
             + CASE WHEN task_id IS NULL THEN 0 ELSE 1 END
            ) = 1),
    CONSTRAINT check_comment_created_at_lte_edited_at
        CHECK (edited_at >= created_at)

);
