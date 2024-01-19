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
    INSERT INTO user_record
        (id, name, email, birth, avatar_url,
        gender, role, status,
        created_at, edited_at)
        VALUES
        (user0_id, 'Dave Null', 'dave@null.com', '1996-06-23', 'dave.jpg',
        'male', 'admin', 'available',
        '2023-12-22 08:38:20.288688', '2023-12-22 08:38:20.288688');

    INSERT INTO user_record
        (id, name, email, birth, avatar_url,
        gender, role, status,
        created_at, edited_at)
        VALUES
        (user1_id, 'Tana Smith', 't.smith@seznam.cz', '1994-02-10', 'tana.jpg',
        'female', 'user', 'available',
        '2023-12-26 07:33:20.288688', '2023-12-26 07:33:20.288688');

    INSERT INTO user_record
        (id, name, email, birth, avatar_url,
        gender, role, status,
        created_at, edited_at)
        VALUES
        (user2_id, 'John Doe', 'doe@gmail.com', '1998-06-23', 'doe.jpg',
        'male', 'user', 'available',
        '2023-12-22 08:38:20.288688', '2023-12-22 08:38:20.288688');

    INSERT INTO company
        (id, name, description,
        website, crn, vatin,
        phone, email, avatar_url,
        created_at, edited_at)
        VALUES
        (company0_id, 'AMD', 'Advanced Micro Devices, Inc.',
        'https://amd.com', 'crn_amd', 'vatin_amd',
        '+1 408-749-4000', 'info@amd.com', 'amd.png',
        '2023-12-22 08:38:20.288688', '2023-12-22 08:38:20.288688');

    INSERT INTO company
        (id, name, description,
        website, crn, vatin,
        phone, email, avatar_url,
        created_at, edited_at)
        VALUES
        (company1_id, 'ReportLab', 'ReportLab Europe Ltd.',
        'https://reportlab.com', 'crn_reportlab', 'vatin_reportlab',
        '+44 20 8191 7277', 'support@reportlab.com', 'reportlab.png',
        '2023-12-24 08:38:20.288688', '2023-12-24 08:38:20.288688');

    INSERT INTO company
        (id, name, description,
        website, crn, vatin,
        phone, email, avatar_url,
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
END $$;