--
-- Tables
--
-- Note: Keep a same table order as that present in the emptying SQL script.
--
DROP TABLE IF EXISTS assigned_staff;
DROP TABLE IF EXISTS associated_company;
DROP TABLE IF EXISTS address;
DROP TABLE IF EXISTS comment;
DROP TABLE IF EXISTS task;
DROP TABLE IF EXISTS event_staff;
DROP TABLE IF EXISTS workday;
DROP TABLE IF EXISTS timesheet;
DROP TABLE IF EXISTS employment;
DROP TABLE IF EXISTS company;
DROP TABLE IF EXISTS user_record;
DROP TABLE IF EXISTS event;

-- Enums
DROP TYPE IF EXISTS acceptance_status;
DROP TYPE IF EXISTS approval_status;
DROP TYPE IF EXISTS association;
DROP TYPE IF EXISTS employment_contract;
DROP TYPE IF EXISTS employee_level;
DROP TYPE IF EXISTS event_role;
DROP TYPE IF EXISTS gender;
DROP TYPE IF EXISTS task_priority;
DROP TYPE IF EXISTS user_role;
DROP TYPE IF EXISTS user_status;