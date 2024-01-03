--
-- A script for deleting all rows from all tables.
--
-- Note: Make sure any change in order of tables gets synced
--       with the SQL deletion script.
--
DELETE FROM assigned_staff;
DELETE FROM associated_company;
DELETE FROM address;
DELETE FROM comment;
DELETE FROM task; -- must be after `comment`
DELETE FROM event_staff; -- must be after `task`
DELETE FROM work_day;
DELETE FROM timesheet; -- must be after `work_day`, before `company`
DELETE FROM employment; -- must be after `event_staff` and `timesheet`
DELETE FROM company; -- must be after `address` and `associated_company`
DELETE FROM user_record;
DELETE FROM event; -- must be after `comment`

