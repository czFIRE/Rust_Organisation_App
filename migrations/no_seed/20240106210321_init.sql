-- Enums

--
-- Keep these enum types ordered alphabetically.
--
CREATE TYPE acceptance_status       AS ENUM ('pending', 'accepted', 'rejected');
CREATE TYPE approval_status         AS ENUM ('not_requested', 'pending',
                                             'accepted', 'rejected');
CREATE TYPE association             AS ENUM ('sponsor', 'organizer', 'media', 'other');
CREATE TYPE employment_contract     AS ENUM ('dpp', 'dpc', 'hpp');
CREATE TYPE employee_level          AS ENUM ('basic', 'manager', 'company_administrator');
CREATE TYPE event_role              AS ENUM ('staff', 'organizer');
CREATE TYPE gender                  AS ENUM ('male', 'female', 'other');
CREATE TYPE task_priority           AS ENUM ('low', 'medium', 'high');
CREATE TYPE user_role               AS ENUM ('user', 'admin');
CREATE TYPE user_status             AS ENUM ('available', 'unavailable');

-- Tables

-- BASIC ENTTITIES

CREATE TABLE user_record
(
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    -------------------------------------------------------
    name        VARCHAR(255) NOT NULL,
    email       VARCHAR(255) NOT NULL UNIQUE,
    birth       DATE NOT NULL,
    avatar_url  VARCHAR(255) NOT NULL DEFAULT 'img/default/user.jpg',
    gender      gender NOT NULL,
    role        user_role NOT NULL DEFAULT 'user',
    status      user_status NOT NULL DEFAULT 'available',
    -------------------------------------------------------
    created_at  TIMESTAMP NOT NULL DEFAULT now(),
    edited_at   TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at  TIMESTAMP,
    -------------------------------------------------------
    CONSTRAINT check_user_record_name_len
        CHECK (char_length(name) >= 1),
    CONSTRAINT check_user_record_email_len
        CHECK (char_length(name) >= 3),
    CONSTRAINT check_user_record_created_at_lte_edited_at
        CHECK (edited_at >= created_at)
);


CREATE TABLE company
(
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    -------------------------------------------------------
    name        VARCHAR(255) NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    website     VARCHAR(255),
    crn         VARCHAR(16) NOT NULL UNIQUE,
    vatin       VARCHAR(18) NOT NULL UNIQUE,
    phone       VARCHAR(255) NOT NULL UNIQUE,
    email       VARCHAR(255) NOT NULL UNIQUE,
    avatar_url  VARCHAR(255) NOT NULL DEFAULT 'img/default/company.jpg',
    -------------------------------------------------------
    created_at  TIMESTAMP NOT NULL DEFAULT now(),
    edited_at   TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at  TIMESTAMP,
    -------------------------------------------------------
    CONSTRAINT check_company_name_len
        CHECK (char_length(name) >= 1),
    CONSTRAINT check_company_crn_len
        CHECK (char_length(crn) >= 1),
    CONSTRAINT check_company_vatin_len
        CHECK (char_length(vatin) >= 1),
    CONSTRAINT check_company_phone_len
        CHECK (char_length(phone) >= 2),
    CONSTRAINT check_company_email_len
        CHECK (char_length(email) >= 3),
    CONSTRAINT check_company_created_at_lte_edited_at
        CHECK (edited_at >= created_at)
);


CREATE TABLE address
(
    company_id  UUID NOT NULL,
    -------------------------------------------------------
    country       VARCHAR(255) NOT NULL,
    region        VARCHAR(255) NOT NULL,
    city          VARCHAR(255) NOT NULL,
    street        VARCHAR(255) NOT NULL,
    street_number VARCHAR(255) NOT NULL,
    postal_code   VARCHAR(255) NOT NULL,
    -------------------------------------------------------
    PRIMARY KEY (company_id),
    FOREIGN KEY (company_id) REFERENCES company (id),
    -------------------------------------------------------
    CONSTRAINT check_address_country_len
        CHECK (char_length(country) >= 1),
    CONSTRAINT check_address_region_len
        CHECK (char_length(region) >= 1),
    CONSTRAINT check_address_city_len
        CHECK (char_length(city) >= 1),
    CONSTRAINT check_address_street_len
        CHECK (char_length(street) >= 1),
    CONSTRAINT check_address_address_number_len
        CHECK (char_length(street_number) >= 1),
    CONSTRAINT check_address_postal_code_len
        CHECK (char_length(postal_code) >= 1)
);


CREATE TABLE employment
(
    user_id     UUID NOT NULL,
    company_id  UUID NOT NULL,
    -------------------------------------------------------
    manager_id  UUID,
    -------------------------------------------------------
    hourly_wage FLOAT NOT NULL,
    start_date  DATE NOT NULL,
    end_date    DATE NOT NULL default '9999-12-31',
    description TEXT,
    type        employment_contract NOT NULL,
    level       employee_level NOT NULL DEFAULT 'basic',
    -------------------------------------------------------
    created_at  TIMESTAMP NOT NULL DEFAULT now(),
    edited_at   TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at  TIMESTAMP,
    -------------------------------------------------------
    PRIMARY KEY (user_id, company_id),
    FOREIGN KEY (user_id) REFERENCES user_record (id),
    FOREIGN KEY (company_id) REFERENCES company (id),
    FOREIGN KEY (manager_id) REFERENCES user_record  (id),
    -------------------------------------------------------
    CONSTRAINT check_employment_hourly_wage_gte_0
        CHECK (hourly_wage >= 0.0),
    CONSTRAINT check_employment_start_date_lte_end_date
        CHECK (start_date <= end_date),
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
    accepts_staff  BOOLEAN NOT NULL DEFAULT true,
    start_date     DATE NOT NULL,
    end_date       DATE NOT NULL,
    avatar_url     VARCHAR(255) NOT NULL DEFAULT 'img/default/event.jpg',
    -------------------------------------------------------
    created_at     TIMESTAMP NOT NULL DEFAULT now(),
    edited_at      TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at     TIMESTAMP,
    -------------------------------------------------------
    CONSTRAINT check_event_name_len
        CHECK (char_length(name) >= 1),
    CONSTRAINT check_event_start_date_lte_end_date
        CHECK (start_date <= end_date),
    CONSTRAINT check_event_created_at_lte_edited_at
        CHECK (edited_at >= created_at)
);


CREATE TABLE associated_company
(
    company_id  UUID NOT NULL,
    event_id    UUID NOT NULL,
    -------------------------------------------------------
    type        association NOT NULL,
    -------------------------------------------------------
    created_at  TIMESTAMP NOT NULL DEFAULT now(),
    edited_at   TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at  TIMESTAMP,
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
    user_id      UUID NOT NULL,
    company_id   UUID NOT NULL,
    event_id     UUID NOT NULL,
    --------------------------------------------------------
    start_date   DATE NOT NULL,
    end_date     DATE NOT NULL,
    total_hours  REAL NOT NULL DEFAULT 0.0,
    is_editable  BOOLEAN NOT NULL DEFAULT true,
    status       approval_status NOT NULL DEFAULT 'not_requested',
    manager_note TEXT,
    -------------------------------------------------------
    created_at   TIMESTAMP NOT NULL DEFAULT now(),
    edited_at    TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at   TIMESTAMP,
    --------------------------------------------------------
    FOREIGN KEY (user_id, company_id)
        REFERENCES employment (user_id, company_id),
    FOREIGN KEY  (event_id) REFERENCES event (id),
    --------------------------------------------------------
    CONSTRAINT check_timesheet_is_editable_iff_not_requested_or_rejected
        CHECK (NOT(is_editable IS TRUE AND status IN ('pending', 'accepted'))),
    CONSTRAINT check_timesheet_total_hours_between_0_and_744
        -- max hours per month: 24.0 * 31.0 = 744.0
        CHECK (total_hours BETWEEN 0.0 AND 744.0),
    CONSTRAINT check_timesheet_start_date_lte_end_date
        CHECK (start_date <= end_date),
    CONSTRAINT check_timesheet_created_at_lte_edited_at
        CHECK (edited_at >= created_at)
);


CREATE TABLE workday
(   
    timesheet_id UUID NOT NULL,
    date         DATE NOT NULL,
    --------------------------------------------------------
    total_hours  REAL NOT NULL DEFAULT 0.0,
    comment      TEXT,
    --------------------------------------------------------
    created_at   TIMESTAMP NOT NULL DEFAULT now(),
    edited_at    TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at   TIMESTAMP,
    --------------------------------------------------------
    PRIMARY KEY  (timesheet_id, date),
    FOREIGN KEY  (timesheet_id) REFERENCES timesheet (id),
    --------------------------------------------------------
    CONSTRAINT check_workday_total_hours_between_0_and_24
        CHECK (total_hours BETWEEN 0.0 AND 24.0),
    CONSTRAINT check_workday_created_at_lte_edited_at
        CHECK (edited_at >= created_at)
);


-- WORKFLOW ENTITIES

CREATE TABLE event_staff
(
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    -------------------------------------------------------
    user_id     UUID NOT NULL,
    company_id  UUID NOT NULL,
    event_id    UUID NOT NULL,
    decided_by  UUID,
    -------------------------------------------------------
    role        event_role NOT NULL DEFAULT 'staff',
    status      acceptance_status NOT NULL DEFAULT 'pending',
    -------------------------------------------------------
    created_at  TIMESTAMP NOT NULL DEFAULT now(),
    edited_at   TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at  TIMESTAMP,
    -------------------------------------------------------
    FOREIGN KEY (user_id, company_id)
        REFERENCES employment (user_id, company_id),
    FOREIGN KEY (event_id) REFERENCES event (id),
    FOREIGN KEY (decided_by) REFERENCES event_staff (id),
    -------------------------------------------------------
    CONSTRAINT check_event_staff_decided_by_null_iff_pending
        CHECK (NOT(decided_by IS NULL AND status != 'pending')),
    CONSTRAINT check_event_staff_created_at_lte_edited_at
        CHECK (edited_at >= created_at)
);


CREATE TABLE task
(
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    -------------------------------------------------------
    event_id    UUID NOT NULL,
    creator_id  UUID NOT NULL,
    -------------------------------------------------------
    title           VARCHAR(255) NOT NULL,
    description     TEXT,
    finished_at     TIMESTAMP,
    priority        task_priority NOT NULL DEFAULT 'medium',
    accepts_staff   BOOLEAN NOT NULL DEFAULT true,
    -------------------------------------------------------
    created_at  TIMESTAMP NOT NULL DEFAULT now(),
    edited_at   TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at  TIMESTAMP,
    -------------------------------------------------------
    FOREIGN KEY (event_id) REFERENCES event (id),
    FOREIGN KEY (creator_id) REFERENCES event_staff (id),
    -------------------------------------------------------
    CONSTRAINT check_task_title_len
        CHECK (char_length(title) >= 1),
    CONSTRAINT check_task_created_at_lte_edited_at
        CHECK (edited_at >= created_at)
);


CREATE TABLE assigned_staff
(
    task_id     UUID NOT NULL,
    staff_id    UUID NOT NULL,
    -------------------------------------------------------
    decided_by  UUID,
    -------------------------------------------------------
    status      acceptance_status NOT NULL DEFAULT 'pending',
    -------------------------------------------------------
    created_at  TIMESTAMP NOT NULL DEFAULT now(),
    edited_at   TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at  TIMESTAMP,
    -------------------------------------------------------
    PRIMARY KEY (task_id, staff_id),
    FOREIGN KEY (task_id) REFERENCES task (id),
    FOREIGN KEY (staff_id) REFERENCES event_staff (id),
    FOREIGN KEY (decided_by) REFERENCES event_staff (id),
    -------------------------------------------------------
    CONSTRAINT check_assigned_staff_decided_by_null_iff_pending
        CHECK (NOT(decided_by IS NULL AND status != 'pending')),
    CONSTRAINT check_assigned_staff_created_at_lte_edited_at
        CHECK (edited_at >= created_at)
);


CREATE TABLE comment
(
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    -------------------------------------------------------
    event_id    UUID,
    task_id     UUID,
    author_id   UUID NOT NULL,
    -------------------------------------------------------
    content     TEXT NOT NULL,
    -------------------------------------------------------
    created_at  TIMESTAMP NOT NULL DEFAULT now(),
    edited_at   TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at  TIMESTAMP,
    -------------------------------------------------------
    FOREIGN KEY (event_id) REFERENCES event (id),
    FOREIGN KEY (task_id) REFERENCES task (id),
    FOREIGN KEY (author_id) REFERENCES user_record (id),
    -------------------------------------------------------
    CONSTRAINT check_comment_single_relation_only
        CHECK (
            (CASE WHEN event_id IS NULL THEN 0 ELSE 1 END
             + CASE WHEN task_id IS NULL THEN 0 ELSE 1 END
            ) = 1),
    CONSTRAINT check_comment_created_at_lte_edited_at
        CHECK (edited_at >= created_at)

);

--
-- todo later: Once this special table gets finilized, put it into ERD as well.
--
-- todo later: We should enforce only one row has valid_to=NULL
--             and no two (valid_from, valid_to) ranges intersect.
--
CREATE TABLE wage_preset
(
	name VARCHAR(32) PRIMARY KEY,
    -------------------------------------------------------
    valid_from  DATE NOT NULL,
    --
    -- Denotes preset's applicability has not expired yet.
    --
    -- Note: At most only one column may have this set to NULL.
    --
    valid_to    DATE,
    description TEXT NOT NULL DEFAULT '',
    currency    VARCHAR(8) NOT NULL,
	monthly_dpp_employee_no_tax_limit REAL NOT NULL,
	monthly_dpp_employer_no_tax_limit REAL NOT NULL,
	monthly_dpc_employee_no_tax_limit REAL NOT NULL,
	monthly_dpc_employer_no_tax_limit REAL NOT NULL,
	health_insurance_employee_tax_pct REAL NOT NULL,
	social_insurance_employee_tax_pct REAL NOT NULL,
	health_insurance_employer_tax_pct REAL NOT NULL,
	social_insurance_employer_tax_pct REAL NOT NULL,
	min_hourly_wage REAL NOT NULL,
	min_monthly_hpp_salary REAL NOT NULL, -- note: not utilized ATM
    -------------------------------------------------------
    created_at  TIMESTAMP NOT NULL DEFAULT now(),
    edited_at   TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at  TIMESTAMP,
    -------------------------------------------------------
    CONSTRAINT check_wage_preset_name_len
        CHECK (char_length(name) >= 1),
    CONSTRAINT check_wage_preset_description_len
        CHECK (char_length(description) >= 1),
    CONSTRAINT check_wage_preset_currency_len
        CHECK (char_length(currency) >= 1),
    CONSTRAINT check_wage_preset_monthly_dpp_employee_no_tax_limit_gte_0
        CHECK (monthly_dpp_employee_no_tax_limit >= 0.0),
    CONSTRAINT check_wage_preset_monthly_dpp_employer_no_tax_limit_gte_0
        CHECK (monthly_dpp_employer_no_tax_limit >= 0.0),
    CONSTRAINT check_wage_preset_monthly_dpc_employee_no_tax_limit_gte_0
        CHECK (monthly_dpc_employee_no_tax_limit >= 0.0),
    CONSTRAINT check_wage_preset_monthly_dpc_employer_no_tax_limit_gte_0
        CHECK (monthly_dpc_employer_no_tax_limit >= 0.0),
    CONSTRAINT check_wage_preset_health_insurance_employee_tax_pct_gte_0
        CHECK (health_insurance_employee_tax_pct >= 0.0),
    CONSTRAINT check_wage_preset_social_insurance_employee_tax_pct_gte_0
        CHECK (social_insurance_employee_tax_pct >= 0.0),
    CONSTRAINT check_wage_preset_health_insurance_employer_tax_pct_gte_0
        CHECK (health_insurance_employer_tax_pct >= 0.0),
    CONSTRAINT check_wage_preset_social_insurance_employer_tax_pct_gte_0
        CHECK (social_insurance_employer_tax_pct >= 0.0),
    CONSTRAINT check_wage_preset_min_hourly_wage_gte_0
        CHECK (min_hourly_wage >= 0.0),
    CONSTRAINT check_wage_preset_min_monthly_hpp_salary_gte_0
        CHECK (min_monthly_hpp_salary >= 0.0),
    CONSTRAINT check_wage_preset_valid_from_le_valid_to
        CHECK (valid_from < valid_to),
    CONSTRAINT check_wage_preset_created_at_lte_edited_at
        CHECK (edited_at >= created_at)
);
