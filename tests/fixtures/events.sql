DECLARE
    event0_id UUID := 'b71fd7ce-c891-410a-9bb4-70fc5c7748f8';
    event1_id UUID :='3f152d12-0bbd-429a-a9c5-28967d6370cc';

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