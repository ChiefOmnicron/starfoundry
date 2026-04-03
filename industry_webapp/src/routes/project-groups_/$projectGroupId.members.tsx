import { Alert, Button, Group, Stack, Table, Title } from '@mantine/core';
import { createFileRoute } from '@tanstack/react-router'
import { EntitySelectorModal } from '@starfoundry/components/selectors/EntitySelectorModal';
import { EveIcon } from '@starfoundry/components/misc/EveIcon';
import { LoadingAnimation } from '@starfoundry/components/misc/LoadingAnimation';
import { LoadingError } from '@starfoundry/components/misc/LoadingError';
import { useDisclosure } from '@mantine/hooks';
import { useEffect, useState } from 'react';
import { FETCH_PROJECT_GROUP, useFetchProjectGroup } from '@starfoundry/components/services/project-group/fetch';
import type { Entity } from '@starfoundry/components/list/EntityList';
import { compareArray, SaveDialog } from '@starfoundry/components/misc/SaveDialog';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { updateMembers } from '@starfoundry/components/services/project-group/updateMembers';

export const Route = createFileRoute(
    '/project-groups_/$projectGroupId/members',
)({
    component: RouteComponent,
})

function RouteComponent() {
    const queryClient = useQueryClient();
    const { projectGroupId } = Route.useParams();
    const [entitySelectorOpened, { open: openEntitySelector, close: closeEntitySelector }] = useDisclosure(false);

    const [selectedMembers, setSelectedMembers] = useState<Entity[]>([]);
    const [originalMembers, setOriginalMembers] = useState<Entity[]>([]);

    const [successfulUpdate, setSuccessfulUpdate] = useState<boolean>();
    const [errorUpdate, setErrorUpdate] = useState<string | undefined>();

    const {
        isError,
        isPending,
        data: projectGroup,
    } = useFetchProjectGroup(projectGroupId);

    const membersMutation = useMutation({
        mutationFn: () => updateMembers(
            projectGroupId,
            selectedMembers.map(x => {
                return {
                    character_id: x.id,
                }
            })
        ),
        onSuccess: () => {
            setErrorUpdate(undefined);
            setSuccessfulUpdate(true);
            queryClient.invalidateQueries({ queryKey: [FETCH_PROJECT_GROUP] });
        },
        onError: (error) => {
            setErrorUpdate(error as any);
            setSuccessfulUpdate(false);
        }
    });

    useEffect(() => {
        if (projectGroup) {
            const members: Entity[] = projectGroup
                .members
                .map(x => {
                    return {
                        id: x.character.character_id,
                        category: 'character',
                        name: x.character.character_name,
                    }
                });

            setSelectedMembers(members);
            setOriginalMembers(members);
        }
    }, [projectGroup])

    if (isPending) {
        return LoadingAnimation();
    }

    if (isError && !projectGroup) {
        return LoadingError();
    }

    const members = () => {
        return selectedMembers
            .sort((a, b) => a.name.localeCompare(b.name))
            .map(x => {
                return <Table.Tr key={x.id}>
                    <Table.Td>
                        <EveIcon
                            id={x.id}
                            category='characters'
                            type='portrait'
                        />
                    </Table.Td>
                    <Table.Td>{x.name}</Table.Td>
                    <Table.Td>
                        TODO: add/edit permissions
                    </Table.Td>
                </Table.Tr>
            })
    }

    const notification = () => {
        if (successfulUpdate) {
            return <Alert
                mt="sm"
                variant='light'
                color='green'
                title='Update successful'
                data-cy="updateSuccessful"
            >
                Updating members successful
            </Alert>;
        } else if (errorUpdate) {
            return <Alert
                mt="sm"
                variant='light'
                color='red'
                title='Update error'
                data-cy="errorUpdate"
                onClose={ () => setErrorUpdate(undefined) }
                withCloseButton
            >
                There was an error while updating. Please try again later.
            </Alert>;
        } else {
            return <></>
        }
    }

    return <>
        { notification() }

        <EntitySelectorModal
            opened={entitySelectorOpened}
            onClose={closeEntitySelector}

            categories={['character']}

            selected={selectedMembers}
            onSelect={(selectedEntities) => {
                setSelectedMembers(selectedEntities);
                closeEntitySelector();
            }}
        />

        <Stack>
            <Group
                justify='space-between'
            >
                <Title order={2}>Sharing</Title>

                <Button
                    onClick={openEntitySelector}
                >
                    Edit Shares
                </Button>
            </Group>

            <Alert color="gray">
                Characters added below will be able to see the group and it's projects.
            </Alert>

            <Table striped>
                <Table.Thead>
                    <Table.Tr>
                        <Table.Th w={32}></Table.Th>
                        <Table.Th>Character</Table.Th>
                        <Table.Th>Permissions</Table.Th>
                    </Table.Tr>
                </Table.Thead>

                <Table.Tbody>{members()}</Table.Tbody>
            </Table>
        </Stack>

        <SaveDialog
            onReset={() => setSelectedMembers(originalMembers)}
            onSave={() => membersMutation.mutate()}
            show={!compareArray(selectedMembers, originalMembers)}
        />
    </>
}
