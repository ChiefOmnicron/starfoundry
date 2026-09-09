CREATE TABLE IF NOT EXISTS project_sub_project(
    project_id      UUID NOT NULL,
    sub_project_id  UUID NOT NULL,

    FOREIGN KEY (sub_project_id)
        REFERENCES project (id)
        ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS project_sub_project_project_id ON project_sub_project(project_id);
