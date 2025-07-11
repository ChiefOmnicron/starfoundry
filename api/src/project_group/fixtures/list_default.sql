INSERT INTO project_group_default_blacklist (project_group_id, type_id) VALUES
('00000000-0000-0000-0000-000000000001', 4051),
('00000000-0000-0000-0000-000000000001', 4246),
('00000000-0000-0000-0000-000000000001', 4247),
('00000000-0000-0000-0000-000000000001', 4312);

INSERT INTO item (category_id, group_id, name, type_id, volume) VALUES
(4, 1136, 'Nitrogen Fuel Block', 4051, 5),
(4, 1136, 'Hydrogen Fuel Block', 4246, 5),
(4, 1136, 'Helium Fuel Block', 4247, 5),
(4, 1136, 'Oxygen Fuel Block', 4312, 5),
(66, 1321, 'Standup Market Hub I', 35892, 5);

INSERT INTO project_group_default_market (project_group_id, structure_id) VALUES
('00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000001');

INSERT INTO structure (id, structure_id, system_id, type_id, security, name, owner) VALUES
('00000000-0000-0000-0000-000000000001', 1337, 30004759, 35892, 'NULLSEC', 'Some Test Structure', 1);

INSERT INTO system (region_id, region_name, constellation_id, constellation_name, system_id, system_name, security) VALUES
(1, 'Test', 1, 'Test', 30004759, '1DQ1-A', 0);
