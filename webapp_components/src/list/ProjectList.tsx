import { ProjectCard } from "@internal/cards/ProjectCard";
import type { ProjectGroup } from "@internal/services/project-group/fetch";
import type { ProjectList } from "@internal/services/projects/list";
import type { Uuid } from "@internal/services/utils";
import { SimpleGrid, Title } from "@mantine/core";

export function ProjectList({
    projects,
}: ProjectListProps) {
    const groups: ProjectGroup[] = [];
    projects
        .map(x => {
            if (!groups.find(y => y.id === x.project_group.id)) {
                groups.push(x.project_group);
            }
        });

    const groupCards = (projectGroupId: Uuid) => {
        return projects
            .filter(x => x.project_group.id === projectGroupId)
            .map(x => <ProjectCard project={x} />);
    }

    const projectGroups = groups
        .map(x => {
            return <>
                <Title
                    order={2}
                    mt='xs'
                >
                    { x.name }
                </Title>

                <SimpleGrid cols={{
                    base: 1,
                    sm: 4,
                }}>
                    { groupCards(x.id) }
                </SimpleGrid>
            </>
        });

    return <>
        { projectGroups }
    </>
}

export type ProjectListProps = {
    projects: ProjectList[];
}
