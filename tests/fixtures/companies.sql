DECLARE
  company0_id UUID := 'b5188eda-528d-48d4-8cee-498e0971f9f5';
  company1_id UUID := '134d5286-5f55-4637-9b98-223a5820a464';
  company2_id UUID := '71fa27d6-6f00-4ad0-8902-778e298aaed2';

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