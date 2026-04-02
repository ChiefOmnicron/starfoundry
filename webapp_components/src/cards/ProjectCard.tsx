import { BadgeWrapper } from "@internal/wrapper/Badge";
import { Card, Flex, Group, Loader, Stack, Text, Title } from "@mantine/core";
import { InternalLink } from "@internal/links/InternalLink";
import { ProjectProgressBar } from "@internal/misc/ProgressBar";
import { useListProjectJobs } from "@internal/services/projects/listJobs";
import type { ProjectListMinimal, ProjectStatus } from "@internal/services/projects/list";

export function ProjectCard({
    project,

    viewLink        = undefined,
    assistantLink   = undefined,
}: ProjectCardProps) {
    const {
        isPending,
        data: jobs,
    } = useListProjectJobs(project.id);

    const status = (status: ProjectStatus) => {
        switch(status) {
            case 'DONE':
                return <BadgeWrapper color="green">Done</BadgeWrapper>;
            case 'IN_PROGRESS':
                return <BadgeWrapper color="blue">In Progress</BadgeWrapper>;
            case 'READY_TO_START':
                return <BadgeWrapper color="orange">Pause</BadgeWrapper>;
            case 'PAUSED':
                return <BadgeWrapper color="yellow">Pause</BadgeWrapper>;
            default:
                return <BadgeWrapper color="gray">Draft</BadgeWrapper>;
        }
    }

    const additionalMessage = () => {
        const groupedJobs = (jobs || []).flatMap(x => x.entries);
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
        const waiting = (jobs || [])
            .flatMap(x => x.entries)
            .filter(x => x.status === 'WAITING_FOR_MATERIALS' || x.status === 'READY_TO_START')
            .length;
        const inProgress = (jobs || [])
            .flatMap(x => x.entries)
            .filter(x => x.status === 'BUILDING')
            .length;
        const done = (jobs || [])
            .flatMap(x => x.entries)
            .filter(x => x.status === 'DONE')
            .length;
        const total = (jobs || [])
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

                    <Group>
                        {
                            status(project.status)
                        }
                        {
                            isPending
                            ?   <Loader color="blue" size="xs" />
                            :   <></>
                        }
                    </Group>
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
                    {
                        project.status === 'DRAFT'
                        ?   <InternalLink
                                to={assistantLink}
                                params={{
                                    projectId: project.id,
                                } as any}
                                content='Open'
                            />
                        :   <InternalLink
                                to={viewLink}
                                params={{
                                    projectId: project.id,
                                } as any}
                                content='Open'
                            />
                    }
                </Flex>
            </Group>

            <div
                style={{
                    width: '100%'
                }}
            >
                <ProjectProgressBar
                    jobs={jobs || []}
                />
            </div>
        </Card>
    </>
}

type ProjectRequiredCardProps = {
    project: ProjectListMinimal;
}

export type ProjectCardAdditionalProps = {
    viewLink?:      string;
    assistantLink?: string;
}

export type ProjectCardProps = ProjectRequiredCardProps & ProjectCardAdditionalProps;
