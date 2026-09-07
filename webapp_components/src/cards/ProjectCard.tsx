import { BaseCard } from "../cards/BaseCard";
import { Flex, Group, Stack, Text, Title } from "@mantine/core";
import { InternalLink } from "../links/InternalLink";
import { ProjectProgressBar } from "../misc/ProjectProgressBar";
import { ProjectStatusBadge } from "../project/ProjectStatusBadge";
import { useListProjectJobs } from "../services/projects/listJobs";
import type { ProjectListMinimal } from "../services/projects/list";

export function ProjectCard({
    project,

    viewLink        = undefined,
    assistantLink   = undefined,
}: ProjectCardProps) {
    const {
        isPending,
        data: jobs,
    } = useListProjectJobs(project.id, {});

    const additionalMessage = () => {
        if (project.status === 'DRAFT' || project.status === 'READY_TO_START') {
            return <div></div>;
        }

        const groupedJobs = (jobs || []).flatMap(x => x.entries);
        const done = groupedJobs.filter(x => x.status === 'DONE');
        const building = groupedJobs.filter(x => x.status === 'BUILDING');
        const waiting = groupedJobs.filter(x => x.status === 'WAITING_FOR_MATERIALS');

        if (done.length === groupedJobs.length) {
            return <Text size='sm' c="green.9">All jobs done</Text>
        } else if (building.length === 0 && waiting.length > 0) {
            return <Text size='sm' c="red.9">No active jobs</Text>
        } else {
            return <div></div>;
        }
    }

    const header = () => {
        return <>
            <Title order={3}>
                { project.name }
            </Title>
        </>
    }

    const body = () => {
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

        return <Stack
            gap="xs"
        >
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

            {
                project.status === 'DRAFT'
                ?   <></>
                :   <Group
                        gap={'xs'}
                    >
                        <Text size='sm' fw={700}>Progress: </Text>
                        <Text size='sm' c="red.9">{ waiting }</Text> /
                        <Text size='sm' c="blue.9">{ inProgress }</Text> /
                        <Text size='sm' c="green.9">{ done }</Text> /
                        <Text size='sm'>{ total }</Text>
                    </Group>
            }

            <Group
                gap={'xs'}
            >
                <Text
                    size='sm'
                    fw={700}
                >
                    Status:
                </Text>
                <Text
                    size='sm'
                >
                    <ProjectStatusBadge
                        status={project.status}
                    />
                </Text>
            </Group>
        </Stack>
    }

    const footer = () => {
        return <Group
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
    }

    const bottom = () => {
        return <div
            style={{
                width: '100%'
            }}
        >
            <ProjectProgressBar
                jobs={jobs || []}
            />
        </div>
    }

    return <>
        <BaseCard
            header={header()}
            footer={footer()}
            bottom={bottom()}

            loading={isPending}
        >
            {body()}
        </BaseCard>
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
