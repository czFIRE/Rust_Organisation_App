DECLARE
    user0_id UUID := '35341253-da20-40b6-96d8-ce069b1ba5d4';
    user1_id UUID := '0465041f-fe64-461f-9f71-71e3b97ca85f';
    user2_id UUID := 'ac9bf689-a713-4b66-a3d0-41faaf0f8d0c';
    user3_id UUID := '51a01dbf-dcd5-43a0-809c-94ed8e61d420';

INSERT INTO user_record
    (id, name, email, birth, avatar_path,
    gender, role, status,
    created_at, edited_at)
    VALUES
    (user0_id, 'Dave Null', 'dave@null.com', '1996-06-23', 'dave.jpg',
    'male', 'admin', 'available',
    '2023-12-22 08:38:20.288688', '2023-12-22 08:38:20.288688');

INSERT INTO user_record
    (id, name, email, birth, avatar_path,
    gender, role, status,
    created_at, edited_at)
    VALUES
    (user1_id, 'Tana Smith', 't.smith@seznam.cz', '1994-02-10', 'tana.jpg',
    'female', 'user', 'available',
    '2023-12-26 07:33:20.288688', '2023-12-26 07:33:20.288688');