--
-- A script deleting all rows from all tables.
--
-- Note: Needs some tweaking, ATM has issues with constraints.
--
DELETE FROM assigned_staff;
DELETE FROM event_staff;
DELETE FROM employment;
DELETE FROM associated_company;
DELETE FROM company;
DELETE FROM address;
DELETE FROM comment;
DELETE FROM user_record;
DELETE FROM work_day;
DELETE FROM timesheet;
DELETE FROM task;
DELETE FROM event;

