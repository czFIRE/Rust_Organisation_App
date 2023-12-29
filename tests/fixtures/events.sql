DO $$
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
    INSERT INTO event
        (id, name, description,
        website, accepts_staff,
        start_date, end_date, avatar_url,
        created_at, edited_at)
        VALUES
        (event0_id, 'Woodstock', 'A legendary music festival.',
        'https://woodstock.com', true,
        '1969-08-15', '1969-08-18', 'woodstock.png', 
        '2023-05-03 10:38:20.4', '2023-12-01 14:30:20.1');
END $$;