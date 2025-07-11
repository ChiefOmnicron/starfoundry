INSERT INTO character (character_id, corporation_id, character_name, corporation_name) VALUES
(3, 3, 'Test', 'Test');

INSERT INTO project_group_member (group_id, character_id, accepted, permission) VALUES
('00000000-0000-0000-0000-000000000001', 3, TRUE, 126); -- everything but owner
