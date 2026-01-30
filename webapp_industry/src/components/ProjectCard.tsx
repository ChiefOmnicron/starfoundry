import { Badge, Card, Flex, Group, SimpleGrid, Stack, Text, Title } from '@mantine/core';
import { InternalLink } from './InternalLink';
import { LoadingAnimation } from './LoadingAnimation';
import { ProjectProgressBar } from '@/routes/projects/-components/ProgressBar';
import { Route as ProjectRoute } from '@/routes/projects_/$projectId.overview';
import { useListProjectJobs } from '@/services/projects/listJobs';
import type { ProjectGroup } from '@/services/project-group/fetch';
import type { ProjectList as ProjectApiList, ProjectList, ProjectStatus } from '@/services/projects/list';
import type { Uuid } from '@/services/utils';

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

export function ProjectCard({
    project,
}: ProjectCardProps) {
    const {
        isPending,
        isFetching,
        data: jobs
    } = useListProjectJobs(project.id);

    if (isPending || isFetching) {
        return <Card
            key={ project.id }
            style={{
                padding: 0
            }}
        >
            <Flex justify={'center'}>
                <LoadingAnimation />
            </Flex>
        </Card>
    }

    const status = (status: ProjectStatus) => {
        switch(status) {
            case 'DONE':
                return <Badge color="green">Done</Badge>;
            case 'IN_PROGRESS':
                return <Badge color="blue">In Progress</Badge>;
            case 'PAUSED':
                return <Badge color="yellow">Pause</Badge>;
            default:
                return <Badge color="gray">Initial</Badge>;
        }
    }

    const additionalMessage = () => {
        const groupedJobs = jobs.flatMap(x => x.entries);
        const done = groupedJobs.filter(x => x.status === 'DONE');
        const building = groupedJobs.filter(x => x.status === 'BUILDING');
        const waiting = groupedJobs.filter(x => x.status === 'WAITING_FOR_MATERIALS');

        if (done.length === groupedJobs.length) {
            return <Text size='sm' c="green.9">All jobs done</Text>
        } else if (building.length === 0 && waiting.length > 0) {
            return <Text size='sm' c="red.9">No active jobs</Text>
        } else {
            return <div></div>
        }
    }

    const card = () => {
        const waiting = jobs
            .flatMap(x => x.entries)
            .filter(x => x.status === 'WAITING_FOR_MATERIALS')
            .length;
        const inProgress = jobs
            .flatMap(x => x.entries)
            .filter(x => x.status === 'BUILDING')
            .length;
        const done = jobs
            .flatMap(x => x.entries)
            .filter(x => x.status === 'DONE')
            .length;
        const total = jobs
            .flatMap(x => x.entries)
            .length;

        return <>
            <Stack
                gap="xs"
            >
                <Group
                    justify='space-between'
                    gap={'xs'}
                >
                    <Title order={3}>
                        { project.name }
                    </Title>

                    {
                        status(project.status)
                    }
                </Group>

                <Group
                    gap={'xs'}
                >
                    <Text
                        size='sm'
                        fw={700}
                    >
                        Orderer:
                    </Text>
                    <Text
                        size='sm'
                    >
                        { project.orderer }
                    </Text>
                </Group>

                <Group
                    gap={'xs'}
                >
                    <Text
                        size='sm'
                        fw={700}
                    >
                        Estimated finish:
                    </Text>
                    <Text
                        size='sm'
                    >
                        TODO
                    </Text>
                </Group>

                <Group
                    gap={'xs'}
                >
                    <Text size='sm' fw={700}>Progress: </Text>
                    <Text size='sm' c="red.9">{ waiting }</Text> /
                    <Text size='sm' c="blue.9">{ inProgress }</Text> /
                    <Text size='sm' c="green.9">{ done }</Text> /
                    <Text size='sm'>{ total }</Text>
                </Group>
            </Stack>
        </>
    }

    return <>
        <Card 
            key={ project.id }
            style={{
                padding: 0
            }}
        >
            <Card.Section
                style={{
                    margin: '10px',
                    height: '100%'
                }}
            >
            { card() }
            </Card.Section>

            <Group
                justify='space-between'
                gap={'xs'}
                style={{
                    backgroundColor: 'rgba(93,93,104, 0.1)',
                    padding: '5px',
                }}
            >
                { additionalMessage() }

                <Flex
                    align='flex-end'
                    justify='flex-end'
                >
                    <InternalLink
                        to={ProjectRoute.to}
                        params={{
                            projectId: project.id,
                        } as any}
                        content='Open'
                    />
                </Flex>
            </Group>

            <div
                style={{
                    width: '100%'
                }}
            >
                <ProjectProgressBar
                    jobs={jobs}
                />
            </div>
        </Card>
    </>
}

export type ProjectListProps = {
    projects: ProjectApiList[];
}

export type ProjectCardProps = {
    project: ProjectApiList;
}
