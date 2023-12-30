
DO $$
--
-- A list of uuids generated using `select gen_random_uuid()`.
-- 
-- Note: We could also generate UUID values automatically using
--       `DEFAULT` constraint and then assign UUID to a variable with the use
--       of `RETURNING id into <variable>`, but this is more convenient
--       as UUIDs will stay same after each execution of this SQL scriopt.
--
DECLARE
  company0_id UUID := 'b5188eda-528d-48d4-8cee-498e0971f9f5';
  company1_id UUID := '134d5286-5f55-4637-9b98-223a5820a464';
  company2_id UUID := '71fa27d6-6f00-4ad0-8902-778e298aaed2';

  user0_id UUID := '35341253-da20-40b6-96d8-ce069b1ba5d4';
  user1_id UUID := '0465041f-fe64-461f-9f71-71e3b97ca85f';
  user2_id UUID := 'ac9bf689-a713-4b66-a3d0-41faaf0f8d0c';
  user3_id UUID := '51a01dbf-dcd5-43a0-809c-94ed8e61d420';

  event0_id UUID := 'b71fd7ce-c891-410a-9bb4-70fc5c7748f8';
  event1_id UUID :='3f152d12-0bbd-429a-a9c5-28967d6370cc';

  timesheet0_id UUID := 'd47e8141-a77e-4d55-a2d5-4a77de24b6d0';
  timesheet1_id UUID := '0f0f0ff5-0073-47cc-bd1f-540a04fee9ea';
  timesheet2_id UUID := 'c51e77aa-bd80-42c7-8b8a-003f018328f6';
  timesheet3_id UUID := '8446b2ba-8223-4388-be5f-9efdfc4ea265';
  timesheet4_id UUID := 'a19a0ac6-3bd2-4ebd-bc8d-ec111ec9f705';
  timesheet5_id UUID := 'ced9f31c-8662-4812-9005-b8ae85d3b951';

  event_staff0_id UUID := '9281b570-4d02-4096-9136-338a613c71cd';
  event_staff1_id UUID := 'a96d1d99-93b5-469b-ac62-654b0cf7ebd3';
  event_staff2_id UUID := 'aa7f3d0e-ab48-473b-ac69-b84cb74f34f7';

  task0_id UUID := '7ae0c017-fe31-4aac-b767-100d18a8877b';
  task1_id UUID := 'bd9b422d-33c1-42a2-88bf-a56ce6cc55a6';

  comment0_id UUID := '0d6cec6a-4fe8-4e44-bf68-e33de0ed121b';
  comment1_id UUID := 'daac23ec-fb36-434a-823b-49716ed2002c';


BEGIN
    INSERT INTO company
        (id, name, description,
    	website, crn, vatin,
    	phone, email, avatar_path,
    	created_at, edited_at)
        VALUES
    	(company0_id, 'AMD', 'Advanced Micro Devices, Inc.',
    	'https://amd.com', 'crn_amd', 'vatin_amd',
    	'+1 408-749-4000', 'info@amd.com', 'amd.png',
    	'2023-12-22 08:38:20.288688', '2023-12-22 08:38:20.288688');

    INSERT INTO company
        (id, name, description,
    	website, crn, vatin,
    	phone, email, avatar_path,
    	created_at, edited_at)
        VALUES
    	(company1_id, 'ReportLab', 'ReportLab Europe Ltd.',
    	'https://reportlab.com', 'crn_reportlab', 'vatin_reportlab',
    	'+44 20 8191 7277', 'support@reportlab.com', 'reportlab.png',
    	'2023-12-24 08:38:20.288688', '2023-12-24 08:38:20.288688');

    INSERT INTO company
        (id, name, description,
    	website, crn, vatin,
    	phone, email, avatar_path,
    	created_at, edited_at)
        VALUES
    	(company2_id, 'Prusa Research', 'Prusa Research a.s.',
    	'https://prusa3d.com', 'CRN_prusa', 'CZ06649114',
    	'123 456 789', 'info@prusa3d.com', 'prusa_design.png',
    	'2023-12-24 15:55:20.288688', '2023-12-24 19:38:20.288688');

    INSERT INTO address
        (company_id, country, region, city,
    	street, street_number, postal_code)
        VALUES
    	(company0_id, 'United States', 'CA', 'Santa Clara',
    	'Augustine Drive', '2485', '95054');

    INSERT INTO address
        (company_id, country, region, city,
    	street, street_number, postal_code)
        VALUES
    	(company1_id, 'United Kingdom', 'Wimbledon', 'London',
    	'Wimbledon Hill Road', '35', 'SW19 7NB');

    INSERT INTO address
        (company_id, country, region, city,
    	street, street_number, postal_code)
        VALUES
    	(company2_id, 'Czech republic', 'Prague', 'Prague',
    	'Partyzanska', '188/7A', '170 00');

    INSERT INTO user_record
        (id, name, email, birth, avatar_path,
    	gender, role, status,
    	created_at, edited_at)
    	VALUES
    	(user0_id, 'Dave Null', 'dave@null.com', '1996-06-23', 'dave.jpg',
    	'male', 'user', 'available',
    	'2023-12-22 08:38:20.288688', '2023-12-22 08:38:20.288688');

    INSERT INTO user_record
        (id, name, email, birth, avatar_path,
    	gender, role, status,
    	created_at, edited_at)
    	VALUES
    	(user1_id, 'Tana Smith', 't.smith@seznam.cz', '1994-02-10', 'tana.jpg',
    	'female', 'user', 'available',
    	'2023-12-26 07:33:20.288688', '2023-12-26 07:33:20.288688');

	INSERT INTO user_record
        (id, name, email, birth, avatar_path,
    	gender, role, status,
    	created_at, edited_at)
    	VALUES
    	(user2_id, 'Annna Smeth', 'a.smeth@sezam.cz', '1998-02-10', 'tana.jpg',
    	'female', 'user', 'available',
    	'2023-12-27 07:33:20.288688', '2023-12-27 07:33:20.288688');

	INSERT INTO user_record
        (id, name, email, birth, avatar_path,
    	gender, role, status,
    	created_at, edited_at)
    	VALUES
    	(user3_id, 'Dee Scord Admen', 'dee@lmao.com', '1999-06-23', 'dave.jpg',
    	'male', 'admin', 'available',
    	'2023-12-20 08:38:20.288688', '2023-12-20 08:38:20.288688');

    INSERT INTO employment
        (user_id, company_id, manager_id, hourly_wage,
    	start_date, end_date, description,
    	type, level,
    	created_at, edited_at)
        VALUES
    	(user0_id, company0_id, NULL, 300,
    	'2023-01-01', '2025-01-01', '-',
    	'HPP', 'company_administrator',
    	'2022-12-29 12:38:20.4', '2023-12-10 14:52:20.1');

	INSERT INTO employment
		(user_id, company_id, manager_id, hourly_wage,
		start_date, end_date, description,
		type, level,
		created_at, edited_at)
		VALUES
		(user1_id, company0_id, user0_id, 200,
		'2023-02-01', '2025-01-01', '-',
		'HPP', 'manager',
		'2023-12-30 15:00:03.4', '2023-12-30 15:00:03.4');

    INSERT INTO event
        (id, name, description,
    	website, accepts_staff,
		start_date, end_date, avatar_path,
    	created_at, edited_at)
        VALUES
    	(event0_id, 'Woodstock', 'A legendary music festival.',
    	'https://woodstock.com', true,
		'1969-08-15', '1969-08-18', 'woodstock.png', 
    	'2023-05-03 10:38:20.4', '2023-12-01 14:30:20.1');

    INSERT INTO associated_company
        (company_id, event_id, type,
    	created_at, edited_at)
        VALUES
    	(company0_id, event0_id, 'organizer',
    	'2023-05-03 10:38:20.4', '2023-12-01 14:30:20.1');

    INSERT INTO timesheet
        (id, user_id, company_id, event_id,
    	start_date, end_date, total_hours,
    	is_editable, status, manager_note,
    	created_at, edited_at)
        VALUES
    	(timesheet0_id, user0_id, company0_id, event0_id,
    	'1969-08-16', '1969-08-17', 22,
    	true, 'not_requested', NULL,
    	'1969-08-16 18:26:0.0', '1969-08-17 20:00:0.0');

    INSERT INTO work_day
        (timesheet_id, date, total_hours, comment, is_editable,
    	created_at, edited_at)
        VALUES
    	(timesheet0_id, '1969-08-15', 12, '', true,
    	'1969-08-16 18:28:5.4', '1969-08-17 08:22:32.4');

    INSERT INTO work_day
        (timesheet_id, date, total_hours, comment, is_editable,
    	created_at, edited_at)
        VALUES
    	(timesheet0_id, '1969-08-16', 10, '', true,
    	'1969-08-17 20:00:0.0', '1969-08-17 20:00:0.0');



    INSERT INTO event_staff
        (id, user_id, company_id, event_id,
    	decided_by, role, status,
    	created_at, edited_at)
        VALUES
    	(event_staff0_id, user0_id, company0_id, event0_id,
    	event_staff0_id, 'organizer', 'accepted',
    	'2023-05-03 10:40:20.1', '2023-05-04 08:11:20.4');

    INSERT INTO task
        (id, event_id, creator_id, title,
    	description, finished_at, priority,
    	accepts_staff, created_at, edited_at)
        VALUES
    	(task0_id, event0_id, event_staff0_id, 'Prepare stage for Joe Cocker',
    	NULL, NULL, 'medium', true,
    	'2023-05-03 10:42:10.4', '2023-05-03 10:42:10.4');

    INSERT INTO assigned_staff
        (task_id, staff_id, decided_by, status,
    	created_at, edited_at)
    	VALUES
    	(task0_id, event_staff0_id, event_staff0_id, 'accepted',
    	'2023-05-03 11:45:10.4', '2023-05-03 11:45:10.4');

    INSERT INTO comment
        (id, event_id, task_id, author_id,
		content,
    	created_at, edited_at)
        VALUES
    	(comment0_id, NULL, task0_id, user0_id,
		'Joe will need 3 guitars on stage.',
    	'2023-05-03 11:55:10.4', '2023-05-03 11:55:10.4');

	INSERT INTO comment
		(id, event_id, task_id, author_id,
		content, created_at, edited_at)
		VALUES
		(comment1_id, event0_id, NULL, user0_id,
		'Mayyyn, this event is amazing!',
		'2023-05-03 10:00:01.5', '2023-05-03 10:05:16.1');
END $$;