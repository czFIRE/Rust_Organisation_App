
DO $$
--
-- A list of uuids generated using `select gen_random_uuid()`.
-- 
-- Note: We could also generate UUID values automatically using
--       `DEFAULT` constraint and then assign UUID to a variable with the use
--       of `RETURNING id into <variable>`, but this is more convenient
--       as UUIDs will stay same after each execution of this SQL script.
--
DECLARE
  company0_id UUID := 'b5188eda-528d-48d4-8cee-498e0971f9f5';
  company1_id UUID := '134d5286-5f55-4637-9b98-223a5820a464';
  company2_id UUID := '71fa27d6-6f00-4ad0-8902-778e298aaed2';

  user0_id UUID := '35341253-da20-40b6-96d8-ce069b1ba5d4';
  user1_id UUID := '0465041f-fe64-461f-9f71-71e3b97ca85f';
  user2_id UUID := 'ac9bf689-a713-4b66-a3d0-41faaf0f8d0c';
  user3_id UUID := '51a01dbf-dcd5-43a0-809c-94ed8e61d420';
  user4_id UUID := '30eff81e-a8d9-4ddc-b92e-e4b58e145920';
  user5_id UUID := 'ced9f31c-8662-4812-9005-b8ae85d3b951';
  user6_id UUID := '4a799b2c-3b5f-41ec-a6e3-442cef915051';
  user7_id UUID := 'ac6ca4f1-0654-4815-b3b3-2fe7c50c173c';
  user8_id UUID := '68d792bb-6c78-4cd5-94c5-8db6f162b586';
  user9_id UUID := '8a4e01c6-a7f7-48dd-be76-f4816a21b888';

  event0_id UUID := 'b71fd7ce-c891-410a-9bb4-70fc5c7748f8';
  event1_id UUID := '3f152d12-0bbd-429a-a9c5-28967d6370cc';
  event2_id UUID := '3f152dad-0bbd-4e9a-aec5-2a567d6370cc';
  event3_id UUID := '2321154f-8c57-4ee2-9493-c9243ae7426a';

  timesheet0_id UUID := 'd47e8141-a77e-4d55-a2d5-4a77de24b6d0';
  timesheet1_id UUID := '0f0f0ff5-0073-47cc-bd1f-540a04fee9ea';
  timesheet2_id UUID := 'c51e77aa-bd80-42c7-8b8a-003f018328f6';
  timesheet3_id UUID := '8446b2ba-8223-4388-be5f-9efdfc4ea265';
  timesheet4_id UUID := 'a19a0ac6-3bd2-4ebd-bc8d-ec111ec9f705';

  event_staff0_id UUID := '9281b570-4d02-4096-9136-338a613c71cd';
  event_staff1_id UUID := 'a96d1d99-93b5-469b-ac62-654b0cf7ebd3';
  event_staff2_id UUID := 'aa7f3d0e-ab48-473b-ac69-b84cb74f34f7';
  event_staff3_id UUID := 'ae228dfb-7265-4059-98c4-c4a1e6233cf4';
  event_staff4_id UUID := '31020fa2-7a24-4d8f-927b-98e26d2929b0';
  event_staff5_id UUID := 'e1b266b8-607f-46d0-9027-2a2d28637f15';

  task0_id UUID := '7ae0c017-fe31-4aac-b767-100d18a8877b';
  task1_id UUID := 'bd9b422d-33c1-42a2-88bf-a56ce6cc55a6';

  comment0_id UUID := '0d6cec6a-4fe8-4e44-bf68-e33de0ed121b';
  comment1_id UUID := 'daac23ec-fb36-434a-823b-49716ed2002c';
  comment2_id UUID := '68c76ba8-86a1-4bc2-8402-2aaa9e4fdb9b';


BEGIN
--------------------------------------------------------------------------------

    INSERT INTO company
        (id, name, description,
        website, crn, vatin,
        phone, email, avatar_url,
        created_at, edited_at)
        VALUES
        (company0_id, 'AMD', 'Advanced Micro Devices, Inc.',
        'https://amd.com', 'crn_amd', 'vatin_amd',
        '+1 408-749-4000', 'info@amd.com', 'amd.png',
        '2023-12-22 08:38', '2023-12-22 08:38');

    INSERT INTO company
        (id, name, description,
        website, crn, vatin,
        phone, email, avatar_url,
        created_at, edited_at)
        VALUES
        (company1_id, 'ReportLab', 'ReportLab Europe Ltd.',
        'https://reportlab.com', 'crn_reportlab', 'vatin_reportlab',
        '+44 20 8191 7277', 'support@reportlab.com', 'reportlab.png',
        '2023-12-24 08:38', '2023-12-24 08:38');

    INSERT INTO company
        (id, name, description,
        website, crn, vatin,
        phone, email, avatar_url,
        created_at, edited_at)
        VALUES
        (company2_id, 'Prusa Research', 'Prusa Research a.s.',
        'https://prusa3d.com', 'CRN_prusa', 'CZ06649114',
        '123 456 789', 'info@prusa3d.com', 'prusa_design.png',
        '2023-12-24 15:55', '2023-12-24 19:38');

--------------------------------------------------------------------------------

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

--------------------------------------------------------------------------------

    INSERT INTO user_record
        (id, name, email,
        birth, avatar_url,
        gender, role, status,
        created_at, edited_at)
        VALUES
        (user0_id, 'Dave Null', 'dave@null.com',
        '1996-06-23', 'dave.jpg',
        'male', 'admin', 'available',
        '2023-12-22 08:38', '2023-12-22 08:38');

    INSERT INTO user_record
        (id, name, email,
        birth, avatar_url,
        gender, role, status,
        created_at, edited_at)
        VALUES
        (user1_id, 'Tana Smith', 't.smith@seznam.cz',
        '1994-02-10', 'tana.jpg',
        'female', 'user', 'available',
        '2023-12-26 07:33', '2023-12-26 07:33');

    INSERT INTO user_record
        (id, name, email,
        birth, avatar_url,
        gender, role, status,
        created_at, edited_at)
        VALUES
        (user2_id, 'Anna Smeth', 'a.smeth@sezam.cz',
        '1998-02-10', 'anna.jpg',
        'female', 'user', 'available',
        '2023-12-27 07:33', '2023-12-27 07:33');

    INSERT INTO user_record
        (id, name, email,
        birth, avatar_url,
        gender, role, status,
        created_at, edited_at)
        VALUES
        (user3_id, 'Dee Scord', 'dee@lmao.com',
        '1999-06-23', 'dee.jpg',
        'male', 'user', 'available',
        '2023-12-20 08:38', '2023-12-20 08:38');

    INSERT INTO user_record
        (id, name, email,
        birth, avatar_url,
        gender, role, status,
        created_at, edited_at)
        VALUES
        (user4_id, 'Joanna Dural', 'j.dural@centrum.cz',
        '1995-08-10', 'joanna.jpg',
        'female', 'user', 'unavailable',
        '2023-12-31 09:33', '2024-01-01 12:33');

    INSERT INTO user_record
        (id, name, email,
        birth, avatar_url,
        gender, role, status,
        created_at, edited_at)
        VALUES
        (user5_id, 'Alan Papalochus', 'alan@pap.cz', '1968-08-01', 'alan.jpg',
        'male', 'user', 'available',
        '2024-01-01 10:33', '2024-01-01 10:33');
    
    INSERT INTO user_record 
	    (id, name, email, birth, 
	     gender, role, status) 
    VALUES (user6_id, 'James Bean', 'jamesbean176@snailmail.com',
            '1999-07-02', 'male', 'user', 'available');

    INSERT INTO user_record 
        (id, name, email, birth, 
        gender, role, status) 
    VALUES (user7_id, 'Rick Grimes', 'python@cowboy.com',
            '1987-05-06', 'male', 'user', 'available');

    INSERT INTO user_record 
        (id, name, email, birth, 
        gender, role, status) 
    VALUES (user8_id, 'Ferris McRustacean', 'crab@rave.com',
            '2015-05-15', 'other', 'user', 'available');

    INSERT INTO user_record 
        (id, name, email, birth, 
        gender, role, status) 
    VALUES (user9_id, 'Borra Checker', 'cannot@borrow.var',
            '2015-05-15', 'female', 'admin', 'available');

--------------------------------------------------------------------------------

    INSERT INTO employment
        (user_id, company_id, manager_id, hourly_wage,
        start_date, end_date, description,
        type, level,
        created_at, edited_at)
        VALUES
        (user0_id, company0_id, NULL, 300,
        '2023-01-01', '2025-01-01', '-',
        'hpp', 'company_administrator',
        '2022-12-29 12:38', '2023-12-10 14:52');

    INSERT INTO employment
        (user_id, company_id, manager_id, hourly_wage,
        start_date, end_date, description,
        type, level,
        created_at, edited_at)
        VALUES
        (user0_id, company1_id, NULL, 300,
        '2023-01-01', '2025-01-01', '-',
        'hpp', 'company_administrator',
        '2022-12-29 12:38', '2023-12-10 14:52');

    INSERT INTO employment
        (user_id, company_id, manager_id, hourly_wage,
        start_date, end_date, description,
        type, level,
        created_at, edited_at)
        VALUES
        (user1_id, company1_id, user0_id, 200,
        '2023-02-01', '2025-01-01', '-',
        'dpc', 'manager',
        '2023-12-30 15:00', '2023-12-30 15:00');

    INSERT INTO employment
        (user_id, company_id, manager_id, hourly_wage,
        start_date, end_date, description,
        type, level,
        created_at, edited_at)
        VALUES
        (user2_id, company1_id, NULL, 280,
        '2023-12-31', '2026-01-01', '-',
        'dpp', 'manager',
        '2023-12-28 14:30', '2023-12-28 14:30');

    INSERT INTO employment
        (user_id, company_id, manager_id, hourly_wage,
        start_date, end_date, description,
        type, level,
        created_at, edited_at)
        VALUES
        (user3_id, company2_id, NULL, 150,
        '2023-01-01', '2025-01-01', '-',
        'dpp', 'company_administrator',
        '2022-12-29 12:38', '2023-12-10 14:52');

    INSERT INTO employment
        (user_id, company_id, manager_id, hourly_wage,
        start_date, end_date, description,
        type, level,
        created_at, edited_at)
        VALUES
        (user5_id, company1_id, NULL, 135,
        '2024-01-01', '2025-01-01', '-',
        'dpc', 'basic',
        '2023-12-30 11:38', '2023-12-31 14:52');

    INSERT INTO employment 
        (user_id, company_id, manager_id, hourly_wage,
        start_date, end_date, description, type, level)
        VALUES (user6_id, company0_id, user0_id, 250.0,
            '2020-01-01', '9999-12-31', 'Concert catering evaluator',
            'hpp', 'basic');
        
    INSERT INTO employment 
        (user_id, company_id, manager_id, hourly_wage,
        start_date, end_date, description, type, level)
        VALUES (user7_id, company0_id, user0_id, 500,
            '2010-10-31', '9999-12-31', 'Head of Security',
            'hpp', 'manager');
        
    INSERT INTO employment 
        (user_id, company_id, manager_id, hourly_wage,
        start_date, end_date, description, type, level)
        VALUES (user8_id, company0_id, user0_id, 200,
            '2024-01-01', '2025-12-31', 'Crab Herder',
            'dpp', 'basic');
        
    INSERT INTO employment 
        (user_id, company_id, manager_id, hourly_wage,
        start_date, end_date, description, type, level)
    VALUES (user8_id, company1_id, user0_id, 200,
	    '2024-01-01', '2025-12-31', 'RAII Ambassador',
	    'dpp', 'basic');

    INSERT INTO employment 
        (user_id, company_id, manager_id, hourly_wage,
        start_date, end_date, description, type, level)
        VALUES (user9_id, company0_id, user0_id, 200,
            '2020-02-01', '2024-12-31', 'Equipment Loan Supervisor',
            'dpc', 'basic');

    INSERT INTO employment 
        (user_id, company_id, manager_id, hourly_wage,
        start_date, end_date, description, type, level)
    VALUES (user8_id, company2_id, user3_id, 800,
        '2019-01-01', '9999-12-31', 'Head of Rave Tech Research',
        'hpp', 'manager');
        
--------------------------------------------------------------------------------

    INSERT INTO event
        (id, name, description,
        website, accepts_staff,
        start_date, end_date, avatar_url,
        created_at, edited_at)
        VALUES
        (event0_id, 'Woodstock', 'A legendary music festival.',
        'https://woodstock.com', true,
        '1969-08-15', '1969-08-18', 'woodstock.png', 
        '2023-05-03 10:38', '2023-12-01 14:30');

    INSERT INTO event
        (id, name, description,
        website, accepts_staff,
        start_date, end_date, avatar_url,
        created_at, edited_at)
        VALUES
        (event1_id, 'Darkness 2024', 'Norwegian festival for happy people.',
        'https://darkness2024.com', true,
        '2024-01-01', '2024-01-03', 'darkness_2024.png',
        '2023-06-06 06:06:6.66', '2023-06-06 16:26:6.66');

    INSERT INTO event
        (id, name, description,
        website, accepts_staff,
        start_date, end_date, avatar_url,
        created_at, edited_at)
        VALUES
        (event2_id, 'Beep Boop 2024', 'An event so controversial no companies associated with it. Please sponsor us?',
        'https://beepboop2024.com', true,
        '2024-01-01', '2024-01-03', 'beep_boop_2024.png',
        '2023-06-06 06:06:6.66', '2023-06-06 16:26:6.66');

    INSERT INTO event
        (id, name,
        description,
        website, accepts_staff,
        start_date, end_date, avatar_url,
        created_at, edited_at)
        VALUES
        (event3_id, 'Elvis revival',
        'A rock-n-roll event possibly ended with arrival of the king',
        'https://elvisforever.org', true,
        '1969-07-28', '1969-08-18', 'elvis_revival.png',
        '2023-09-01 09:06', '2023-09-01 09:06');

--------------------------------------------------------------------------------

    INSERT INTO associated_company
        (company_id, event_id, type,
        created_at, edited_at)
        VALUES
        (company0_id, event0_id, 'organizer',
        '2023-05-03 10:38', '2023-12-01 14:30');

    INSERT INTO associated_company
        (company_id, event_id, type,
        created_at, edited_at)
        VALUES
        (company0_id, event1_id, 'media',
        '2023-12-14 08:38', '2023-12-14 08:38');

    INSERT INTO associated_company
        (company_id, event_id, type,
        created_at, edited_at)
        VALUES
        (company1_id, event1_id, 'other',
        '2023-12-12 07:38', '2023-12-12 07:38');

    INSERT INTO associated_company
        (company_id, event_id, type,
        created_at, edited_at)
        VALUES
        (company1_id, event3_id, 'organizer',
        '2023-09-02 12:38', '2023-12-01 15:32');

    INSERT INTO associated_company
        (company_id, event_id, type,
        created_at, edited_at)
        VALUES
        (company2_id, event1_id, 'sponsor',
        '2023-12-12 12:38', '2023-12-12 12:38');

    INSERT INTO associated_company
        (company_id, event_id, type,
        created_at, edited_at)
        VALUES
        (company2_id, event0_id, 'other',
        '2023-12-12 12:39', '2023-12-12 12:39');

    INSERT INTO associated_company
        (company_id, event_id, type,
        created_at, edited_at)
        VALUES
        (company0_id, event2_id, 'organizer',
        '2023-06-06 10:38', '2023-06-07 14:30');

--------------------------------------------------------------------------------

    INSERT INTO timesheet
        (id, user_id, company_id, event_id,
        start_date, end_date, total_hours,
        is_editable, status, manager_note,
        created_at, edited_at)
        VALUES
        (timesheet0_id, user2_id, company1_id, event0_id,
        '1969-08-15', '1969-08-16', 22,
        true, 'not_requested', NULL,
        '1969-08-15 18:26', '1969-08-16 20:00');

    INSERT INTO timesheet
        (id, user_id, company_id, event_id,
        start_date, end_date, total_hours,
        is_editable, status, manager_note,
        created_at, edited_at)
        VALUES
        (timesheet1_id, user1_id, company1_id, event0_id,
        '1969-08-14', '1969-08-17', 24,
        false, 'accepted', 'Everything seems all righty. Good job.',
        '2023-05-05 10:39', '2023-05-09 13:39');

    INSERT INTO timesheet
        (id, user_id, company_id, event_id,
        start_date, end_date, total_hours,
        is_editable, status, manager_note,
        created_at, edited_at)
        VALUES
        (timesheet2_id, user3_id, company2_id, event1_id,
        '2024-01-01', '2024-01-02', 15,
        false, 'pending', NULL,
        '2024-01-03 18:26', '2024-01-03 18:29');

    INSERT INTO timesheet
        (id, user_id, company_id, event_id,
        start_date, end_date, total_hours,
        is_editable, status, manager_note,
        created_at, edited_at)
        VALUES
        (timesheet3_id, user1_id, company1_id, event3_id,
        '1969-07-28', '1969-08-18', 68,
        false, 'accepted', 'Outstanding performance. As always.',
        '2023-11-05 11:39', '2023-11-05 11:39');

--------------------------------------------------------------------------------

    INSERT INTO workday
        (timesheet_id, date, total_hours, comment, is_editable,
        created_at, edited_at)
        VALUES
        (timesheet0_id, '1969-08-15', 12, '', true,
        '1969-08-16 18:28', '1969-08-17 08:22');

    INSERT INTO workday
        (timesheet_id, date, total_hours, comment, is_editable,
        created_at, edited_at)
        VALUES
        (timesheet0_id, '1969-08-16', 10, '', true,
        '1969-08-17 20:00', '1969-08-17 20:00');

    INSERT INTO workday
        (timesheet_id, date, total_hours, comment, is_editable,
        created_at, edited_at)
        VALUES
        (timesheet1_id, '1969-08-14', 10, '', false,
        '1969-08-17 19:58', '1969-08-17 19:59');

    INSERT INTO workday
        (timesheet_id, date, total_hours, comment, is_editable,
        created_at, edited_at)
        VALUES
        (timesheet1_id, '1969-08-15', 4.5, 'I was overworked as a mule!', false,
        '1969-08-17 20:00', '1969-08-17 20:00');

    INSERT INTO workday
        (timesheet_id, date, total_hours, comment, is_editable,
        created_at, edited_at)
        VALUES
        (timesheet1_id, '1969-08-16', 9.5, '', false,
        '1969-08-17 20:40', '1969-08-17 21:00');

    INSERT INTO workday
        (timesheet_id, date, total_hours, comment, is_editable,
        created_at, edited_at)
        VALUES
        (timesheet2_id, '2024-01-01', 8, '', false,
        '2024-01-01 22:33', '2024-01-01 22:33');

    INSERT INTO workday
        (timesheet_id, date, total_hours, comment, is_editable,
        created_at, edited_at)
        VALUES
        (timesheet2_id, '2024-01-02', 7, '', false,
        '2024-01-02 20:10', '2024-01-02 21:12');

    INSERT INTO workday
        (timesheet_id, date, total_hours, comment, is_editable,
        created_at, edited_at)
        VALUES
        (timesheet3_id, '1969-07-28', 11.5, '', true,
        '1969-07-28 23:00', '1969-07-28 23:00');

    INSERT INTO workday
        (timesheet_id, date, total_hours, comment, is_editable,
        created_at, edited_at)
        VALUES
        (timesheet3_id, '1969-07-30', 11.5, '', true,
        '1969-07-30 23:00', '1969-07-30 23:00');

    INSERT INTO workday
        (timesheet_id, date, total_hours, comment, is_editable,
        created_at, edited_at)
        VALUES
        (timesheet3_id, '1969-08-10', 14, '', true,
        '1969-08-10 23:00', '1969-08-10 23:00');

    INSERT INTO workday
        (timesheet_id, date, total_hours, comment, is_editable,
        created_at, edited_at)
        VALUES
        (timesheet3_id, '1969-08-11', 12, '', true,
        '1969-08-11 23:00', '1969-08-11 23:00');

    INSERT INTO workday
        (timesheet_id, date, total_hours, comment, is_editable,
        created_at, edited_at)
        VALUES
        (timesheet3_id, '1969-08-12', 12, '', true,
        '1969-08-12 23:00', '1969-08-12 23:00');

    INSERT INTO workday
        (timesheet_id, date, total_hours, comment, is_editable,
        created_at, edited_at)
        VALUES
        (timesheet3_id, '1969-08-13', 14, '', true,
        '1969-08-13 23:00', '1969-08-13 23:00');

--------------------------------------------------------------------------------

    INSERT INTO event_staff
        (id, user_id, company_id, event_id,
        decided_by, role, status,
        created_at, edited_at)
        VALUES
        (event_staff0_id, user0_id, company0_id, event0_id,
        event_staff0_id, 'organizer', 'accepted',
        '2023-05-03 10:40', '2023-05-04 08:11');

    INSERT INTO event_staff
        (id, user_id, company_id, event_id,
        decided_by, role, status,
        created_at, edited_at)
        VALUES
        (event_staff1_id, user1_id, company1_id, event0_id,
        event_staff0_id, 'staff', 'accepted',
        '2023-05-03 10:40', '2023-05-04 08:11');

    INSERT INTO event_staff
        (id, user_id, company_id, event_id,
        decided_by, role, status,
        created_at, edited_at)
        VALUES
        (event_staff2_id, user2_id, company1_id, event1_id,
        event_staff2_id, 'organizer', 'accepted',
        '2023-06-01 11:40', '2023-06-01 11:40');

    INSERT INTO event_staff
        (id, user_id, company_id, event_id,
        decided_by, role, status,
        created_at, edited_at)
        VALUES
        (event_staff3_id, user3_id, company2_id, event1_id,
        event_staff2_id, 'staff', 'pending',
        '2023-06-02 11:45', '2023-06-03 19:41');

    INSERT INTO event_staff
        (id, user_id, company_id, event_id,
        decided_by, role, status,
        created_at, edited_at)
        VALUES
        (event_staff4_id, user5_id, company1_id, event1_id,
        event_staff2_id, 'staff', 'accepted',
        '2024-01-01 11:45', '2024-01-01 11:45');

    INSERT INTO event_staff
        (id, user_id, company_id, event_id,
        decided_by, role, status,
        created_at, edited_at)
        VALUES
        (event_staff5_id, user1_id, company1_id, event3_id,
        event_staff5_id, 'organizer', 'accepted',
        '2023-09-03 12:38', '2023-09-03 12:38');

--------------------------------------------------------------------------------

    INSERT INTO task
        (id, event_id, creator_id, title,
        description,
        finished_at, priority, accepts_staff,
        created_at, edited_at)
        VALUES
        (task0_id, event0_id, event_staff0_id, 'Prepare stage for Joe Cocker',
        NULL,
        NULL, 'medium', true,
        '2023-05-03 10:42', '2023-05-03 10:42');

    INSERT INTO task
        (id, event_id, creator_id, title,
        description,
        finished_at, priority, accepts_staff,
        created_at, edited_at)
        VALUES
        (task1_id, event1_id, event_staff2_id, 'Unpack all guitars.',
        'The band SharpBoots must have all guitars unpacked 6+ hours prior.',
        NULL, 'high', true,
        '2023-12-30 11:42', '2023-12-30 11:42');

--------------------------------------------------------------------------------

    INSERT INTO assigned_staff
        (task_id, staff_id, decided_by, status,
        created_at, edited_at)
        VALUES
        (task0_id, event_staff0_id, event_staff0_id, 'accepted',
        '2023-05-03 11:45', '2023-05-03 11:45');

    INSERT INTO assigned_staff
        (task_id, staff_id, decided_by, status,
        created_at, edited_at)
        VALUES
        (task1_id, event_staff2_id, event_staff2_id, 'accepted',
        '2023-05-03 11:45', '2023-05-03 11:45');

    INSERT INTO assigned_staff
        (task_id, staff_id, decided_by, status,
        created_at, edited_at)
        VALUES
        (task1_id, event_staff4_id, NULL, 'pending',
        '2024-01-01 11:45', '2024-01-01 11:45');

--------------------------------------------------------------------------------

    INSERT INTO comment
        (id, event_id, task_id, author_id,
        content,
        created_at, edited_at)
        VALUES
        (comment0_id, NULL, task0_id, user0_id,
        'Joe will need 3 guitars on stage.',
        '2023-05-03 11:55', '2023-05-03 11:55');

    INSERT INTO comment
        (id, event_id, task_id, author_id,
        content, created_at, edited_at)
        VALUES
        (comment1_id, event0_id, NULL, user0_id,
        'Mayyyn, this event is amazing!',
        '2023-05-03 10:00', '2023-05-03 10:05');

    INSERT INTO comment
        (id, event_id, task_id, author_id,
        content, created_at, edited_at)
        VALUES
        (comment2_id, event1_id, NULL, user2_id,
        'This gets organized for the 3rd time ;)',
        '2023-05-03 10:00', '2023-05-03 10:05');

--------------------------------------------------------------------------------

    INSERT INTO wage_preset
        (name,
        valid_from,
        valid_to,
        description,
        currency,
        monthly_dpp_employee_no_tax_limit,
        monthly_dpp_employer_no_tax_limit,
        monthly_dpc_employee_no_tax_limit,
        monthly_dpc_employer_no_tax_limit,
        health_insurance_employee_tax_pct, social_insurance_employee_tax_pct,
        health_insurance_employer_tax_pct, social_insurance_employer_tax_pct,
        min_hourly_wage,
        min_monthly_hpp_salary,
        created_at, edited_at)
    VALUES
        ('cz_1966-01-01',
        '1966-01-01',
        '1992-12-31',
        'An imaginary wage params valid for CSR starting from 1966/01/01',
        'CSK',
        5000,
        5000,
        2000,
        2000,
        4.5, 6.5,
        24.8, 9.0,
        60,
        10000,
        '2023-12-14 12:00', '2023-12-14 12:00');

    INSERT INTO wage_preset
        (name,
        valid_from,
        valid_to,
        description,
        currency,
        monthly_dpp_employee_no_tax_limit,
        monthly_dpp_employer_no_tax_limit,
        monthly_dpc_employee_no_tax_limit,
        monthly_dpc_employer_no_tax_limit,
        health_insurance_employee_tax_pct, social_insurance_employee_tax_pct,
        health_insurance_employer_tax_pct, social_insurance_employer_tax_pct,
        min_hourly_wage,
        min_monthly_hpp_salary,
        created_at, edited_at)
    VALUES
        ('cz_2020-01-01',
        '2020-01-01',
        '2023-12-31',
        'An imaginary wage params valid for Czech republic starting from 2020/01/01',
        'CZK',
        8000,
        8000,
        3500,
        3500,
        4.5, 6.5,
        24.8, 9.0,
        100.0,
        15500,
        '2023-12-16 08:00', '2023-12-16 08:00');

    INSERT INTO wage_preset
        (name,
        valid_from,
        valid_to,
        description,
        currency,
        monthly_dpp_employee_no_tax_limit,
        monthly_dpp_employer_no_tax_limit,
        monthly_dpc_employee_no_tax_limit,
        monthly_dpc_employer_no_tax_limit,
        health_insurance_employee_tax_pct, social_insurance_employee_tax_pct,
        health_insurance_employer_tax_pct, social_insurance_employer_tax_pct,
        min_hourly_wage,
        min_monthly_hpp_salary,
        created_at, edited_at)
    VALUES
        ('cz_2024-01-01',
        '2024-01-01',
        NULL,
        'A wage params valid for Czech republic starting from 2024/01/01',
        'CZK',
        10000,
        10000,
         4000,
         4000,
         4.5, 6.5,
         24.8, 9.0,
         118.13,
         18900,
        '2023-12-16 14:00', '2023-12-16 14:00');

--------------------------------------------------------------------------------

END $$;