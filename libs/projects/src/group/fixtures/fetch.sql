INSERT INTO character (character_id, corporation_id, character_name, corporation_name) VALUES
(1, 1, 'Test', 'Test');

INSERT INTO structure_group (id, owner, name) VALUES
('00000000-0000-0000-0000-000000000000', 1, 'Test');

INSERT INTO project_group (id, owner, name, description) VALUES
('00000000-0000-0000-0000-000000000001', 1, 'First', 'Description');

INSERT INTO project_group_member (group_id, character_id, accepted, permission) VALUES
('00000000-0000-0000-0000-000000000001', 1, TRUE, 1);

INSERT INTO project (owner, name, orderer, project_group_id, structure_group_id) VALUES
(1, 'Test', 'Test', '00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000000');
