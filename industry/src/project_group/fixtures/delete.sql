INSERT INTO project_group (id, owner, name, description) VALUES
('00000000-0000-0000-0000-000000000010', 1, 'DeleteME', 'Description');

INSERT INTO project_group_member (project_group_id, character_id, accepted, permission) VALUES
('00000000-0000-0000-0000-000000000001', 3, TRUE, 126), -- everything but owner
('00000000-0000-0000-0000-000000000010', 1, TRUE, 1);
