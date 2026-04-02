import { ProjectCard, type ProjectCardAdditionalProps } from "@internal/cards/ProjectCard";
import { SimpleGrid, Title } from "@mantine/core";
import type { ProjectGroupMinimal } from "@internal/services/project-group/list";
import type { ProjectListMinimal } from "@internal/services/projects/list";
import type { Uuid } from "@internal/services/utils";

export function ProjectList({
    projects,

    projectCardProps,
}: ProjectListProps) {
    const groups: ProjectGroupMinimal[] = [];
    projects
        .map(x => {
            if (!groups.find(y => y.id === x.project_group.id)) {
                groups.push(x.project_group);
            }
        });

    const groupCards = (projectGroupId: Uuid) => {
        return projects
            .filter(x => x.project_group.id === projectGroupId)
            .map(x => <ProjectCard
                project={x}
                {...projectCardProps}
            />);
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
    projects: ProjectListMinimal[];

    projectCardProps?: ProjectCardAdditionalProps;
}
