INSERT INTO structure (id, structure_id, system_id, type_id, security, name, owner) VALUES
('00000000-0000-0000-0000-000000000001', 1337, 30004759, 35892, 'NULLSEC', 'Some Test Structure', 1),
('00000000-0000-0000-0000-000000000002', 1337, 30004759, 35892, 'NULLSEC', 'Another Test Structure', 2),
('00000000-0000-0000-0000-000000000003', 1337, 30004759, 35892, 'NULLSEC', 'Filter', 1);

INSERT INTO system (region_id, region_name, constellation_id, constellation_name, system_id, system_name, security) VALUES
(1, 'Test', 1, 'Test', 30004759, '1DQ1-A', 0);

INSERT INTO item (category_id, group_id, name, type_id, volume) VALUES
(66, 1321, 'Standup Market Hub I', 35892, 5);
