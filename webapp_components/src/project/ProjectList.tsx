import { ProjectCard, type ProjectCardAdditionalProps } from "../cards/ProjectCard";
import { SimpleGrid, Title } from "@mantine/core";
import type { ProjectGroupMinimal } from "../services/project-group/list";
import type { ProjectListMinimal } from "../services/projects/list";
import type { Uuid } from "../services/utils";

export function ProjectList({
    projects,

    groupByProjectGroup = true,

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
                key={x.id}
                project={x}
                {...projectCardProps}
            />);
    }

    if (groupByProjectGroup) {
        return groups
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
    } else {
        return <>
            <SimpleGrid cols={{
                base: 1,
                sm: 4,
            }}>
                {
                    projects
                        .map(x => <ProjectCard
                            project={x}
                            {...projectCardProps}
                        />)
                }
            </SimpleGrid>
        </>
    }
}

export type ProjectListProps = {
    projects: ProjectListMinimal[];

    groupByProjectGroup?: boolean;

    projectCardProps?: ProjectCardAdditionalProps;
}
