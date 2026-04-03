import { createFileRoute } from '@tanstack/react-router'
import { LoadingAnimation } from '@starfoundry/components/misc/LoadingAnimation';
import { LoadingError } from '@starfoundry/components/misc/LoadingError';
import { MarkdownView } from '@starfoundry/components/detailView/MarkdownView';
import { Stack, Table, Title } from '@mantine/core';
import { useFetchProjectGroup } from '@starfoundry/components/services/project-group/fetch';

export interface QueryParams {
    created?: boolean;
}

export const Route = createFileRoute(
    '/project-groups_/$projectGroupId/overview',
)({
    component: RouteComponent,
    validateSearch: (params: {
        created: boolean,
    }): QueryParams => {
        return {
            created: (params.created) || undefined
        };
    }
})

function RouteComponent() {
    const { projectGroupId } = Route.useParams();

    const {
        isError,
        isPending,
        data: projectGroup,
    } = useFetchProjectGroup(projectGroupId);

    if (isPending) {
        return LoadingAnimation();
    }

    if (isError && !projectGroup) {
        return LoadingError();
    }

    const showDescription = () => {
        if (projectGroup.description) {
            return <>
                <Title order={2}>Description</Title>
                <MarkdownView content={projectGroup.description} />
            </>;
        } else {
            return <></>
        }
    }

    return <>
        TODO: find something to put here

        <Stack>
            <Title order={2}>General</Title>
            <Table variant="vertical" layout="fixed" withTableBorder>
                <Table.Tbody>
                    <Table.Tr>
                        <Table.Th w={160}>Project Count</Table.Th>
                        <Table.Td>{projectGroup.project_count}</Table.Td>
                    </Table.Tr>

                    <Table.Tr>
                        <Table.Th>Members</Table.Th>
                        <Table.Td>{projectGroup.members.length}</Table.Td>
                    </Table.Tr>
                </Table.Tbody>
            </Table>

            {showDescription()}
        </Stack>
    </>
}
