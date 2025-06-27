ALTER TABLE project_group_member ADD COLUMN permission INTEGER NOT NULL DEFAULT 2;

UPDATE project_group_member
SET permission = permission + 1
WHERE character_id = (
    SELECT owner
    FROM project_group
    WHERE id = group_id
);

UPDATE project_group_member
SET permission = permission + 4
WHERE projects = 'WRITE';

UPDATE project_group_member
SET permission = permission + 8
WHERE structures = 'WRITE';

UPDATE project_group_member
SET permission = permission + 16
WHERE project_group = 'WRITE';

ALTER TABLE project_group_member DROP COLUMN projects;
ALTER TABLE project_group_member DROP COLUMN structures;
ALTER TABLE project_group_member DROP COLUMN project_group;

ALTER TABLE project_group_member ALTER COLUMN permission DROP DEFAULT;
ALTER TABLE project ALTER COLUMN project_group_id DROP NOT NULL;
ALTER TABLE project ALTER COLUMN project_group_id DROP DEFAULT;

CREATE TABLE IF NOT EXISTS project_group_permission (
    bit     INTEGER NOT NULL,
    name    VARCHAR NOT NULL,
    comment VARCHAR NOT NULL,

    PRIMARY KEY (bit)
);
INSERT INTO project_group_permission (bit, name, comment) VALUES
(1, 'Owner', 'Owner of the project group, has all permissions'),
(2, 'Read Group', 'Permission to read everything from the group'),
(4, 'Write Project', 'Allows to use the group to create new projects'),
(8, 'Write Structure', 'Allows to add additional structures to the group'),
(16, 'Write Defaults', 'Allows to update the group defaults'),
(32, 'Write Members', 'Allows to add additional members to the group'),
(64, 'Write Group', 'Allows to perform updates on the group itself')
ON CONFLICT DO NOTHING;
