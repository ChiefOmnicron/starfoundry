import { Alert, Button, Flex, Stack, TextInput, Title } from '@mantine/core';
import { createFileRoute, useNavigate } from '@tanstack/react-router'
import { deleteIndustryHub } from '@/services/industry-hub/delete';
import { DeleteResource } from '@/components/DeleteResource';
import { FETCH_INDUSTRY_HUB } from '@/services/industry-hub/fetch';
import { LIST_INDUSTRY_HUB } from '@/services/industry-hub/list';
import { LoadingAnimation } from '@/components/LoadingAnimation';
import { LoadingError } from '@/components/LoadingError';
import { Route as StructureListRoute } from '@/routes/structures/index';
import { StructureLayout } from '@/components/StructureLayout';
import { StructureList } from '@/components/StructureList';
import { StructureSelectorModal } from '@/components/selectors/StructureSelectorModal';
import { updateIndustryHub, type UpdateIndustryHub } from '@/services/industry-hub/update';
import { useDisclosure } from '@mantine/hooks';
import { useEffect, useState } from 'react';
import { useFetchIndustryHub } from '@/services/industry-hub/fetch';
import { useForm } from '@tanstack/react-form';
import { useListStructure, type Structure } from '@/services/structure/list';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { EntitySelectorModal } from '@/components/selectors/EntitySelectorModal';
import { CharacterCorporationAllianceList, type CharacterCorporationAlliance } from '@/components/EntityList';

interface QueryParams {
    created?: boolean;
}

export const Route = createFileRoute('/industry-hubs_/$industryHubId/')({
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
    const { industryHubId } = Route.useParams();
    const { created: createdResource } = Route.useSearch();
    const [entitySelectorOpened, { open: openEntitySelector, close: closeEntitySelector }] = useDisclosure(false);
    const [structureSelectorOpened, { open: openStructureSelector, close: closeStructureSelector }] = useDisclosure(false);

    const navigation = useNavigate();
    const queryClient = useQueryClient();

    const [successfulUpdate, setSuccessfulUpdate] = useState<boolean>();
    const [successfulDelete, setSuccessfulDelete] = useState<boolean>();
    const [errorDelete, setErrorDelete] = useState<string | undefined>();
    const [errorUpdate, setErrorUpdate] = useState<string | undefined>();

    const [selectedEntities, setSelectedEntities] = useState<CharacterCorporationAlliance[]>([]);
    const [selectedStructures, setSelectedStructures] = useState<Structure[]>([]);

    const {
        isPending,
        isError,
        data: industryHub,
    } = useFetchIndustryHub(industryHubId);

    const {
        isPending: isPendingStructures,
        isError: isErrorStructures,
        data: structures,
    } = useListStructure({});

    const mutationUpdate = useMutation({
        mutationFn: (data: UpdateIndustryHub) => updateIndustryHub(industryHubId, data),
        onSuccess: () => {
            setSuccessfulUpdate(true);
            queryClient.invalidateQueries({ queryKey: [FETCH_INDUSTRY_HUB, industryHubId] });
            queryClient.invalidateQueries({ queryKey: [LIST_INDUSTRY_HUB] });
        },
    });

    const mutationDelete = useMutation({
        mutationFn: () => deleteIndustryHub(industryHubId),
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: [LIST_INDUSTRY_HUB] });
            navigation({
                to: StructureListRoute.to,
                search: {
                    deleted: true,
                }
            });
        },
    });

    if (isPending) {
        return LoadingAnimation();
    }

    if (isError) {
        return LoadingError();
    }

    useEffect(() => {
        setSelectedStructures(industryHub.structures);

        let shares = industryHub
            .shares
            .map(x => {
                return {
                    id: x.share_id,
                    name: x.name,
                    category: x.share_type.toLowerCase() as any,
                }
            });
        setSelectedEntities(shares);
    }, [industryHub]);

    const form = useForm({
        defaultValues: {
            name:       industryHub.name,
            structures: industryHub.structures.map(x => x.id),
        },
        onSubmit: async ({ value }) => {
            console.log(value)
            return await mutationUpdate
                .mutateAsync({
                    ...value,
                    structures: selectedStructures
                                    .map(x => x.id),
                    shares:     selectedEntities
                                    .map(x => {
                                        return {
                                            name: x.name,
                                            share_id: x.id,
                                            share_type: x.category.toUpperCase() as any,
                                        }
                                    }),
                })
                .catch(error => {
                    setErrorUpdate(error);
                });
        }
    });

    // needs to be here, to make sure the effect hook is called
    if (isPendingStructures) {
        return LoadingAnimation();
    }

    if (isErrorStructures) {
        return LoadingError();
    }

    const deleteResource = async () => {
        await mutationDelete
            .mutateAsync()
            .catch(error => {
                setErrorDelete(error);
                setSuccessfulDelete(false);
            });
    }

    const notification = () => {
        if (createdResource) {
            return <Alert
                mt="sm"
                variant='light'
                color='green'
                title='Create successful'
                data-cy="createdSuccessfully"
            >
                The structure was successfully created
            </Alert>;
        } else if (successfulUpdate) {
            return <Alert
                mt="sm"
                variant='light'
                color='green'
                title='Update successful'
                data-cy="successfulUpdate"
                onClose={ () => setSuccessfulUpdate(false) }
                withCloseButton
            >
                The structure was updated
            </Alert>;
        } else if (successfulDelete) {
            return <Alert
                mt="sm"
                variant='light'
                color='green'
                title='Delete successful'
                data-cy="successfulUpdate"
                onClose={ () => setSuccessfulDelete(false) }
                withCloseButton
            >
                The structure was deleted
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
        } else if (errorDelete) {
            return <Alert
                mt="sm"
                variant='light'
                color='red'
                title='Delete error'
                data-cy="errorDelete"
                onClose={ () => setErrorDelete(undefined) }
                withCloseButton
            >
                There was an error while deleting. Please try again later.
            </Alert>;
        }
    };

    const dangerZone = () => {
        return <>
            <Title
                data-cy="danger-zone-header"
                order={2}
                mt="md"
            >
                Danger Zone
            </Title>

            <DeleteResource
                resource={ industryHub.name }
                onConfirm={ () => deleteResource() }
            />
        </>
    }

    const structureLayouts = () => {
        let systems: TmpSystem[] = [];
        selectedStructures
            .forEach(x => {
                if (!systems.find(y => y.systemId === x.system.system_id)) {
                    let system = {
                        systemId: x.system.system_id,
                        systemName: x.system.system_name
                    };
                    systems.push(system);
                }
            });
        systems.sort((a: TmpSystem, b: TmpSystem) => a.systemName.localeCompare(b.systemName));
        
        let elements = [];
        for (const system of systems) {
            const structures = selectedStructures
                .filter(x => x.system.system_id === system.systemId);
            elements.push(<>
                <Title order={4}>{ system.systemName }</Title>

                <StructureLayout
                    structures={structures}
                />
            </>);
        }

        return elements;
    }

    const onSelectStructure = (structures: Structure[]) => {
        setSelectedStructures(structures);
        closeStructureSelector();
    }

    return <>
        { notification() }

        <EntitySelectorModal
            opened={entitySelectorOpened}
            onClose={closeEntitySelector}

            selected={selectedEntities}
            onSelect={(selectedEntities) => {
                setSelectedEntities(selectedEntities);
                closeEntitySelector();
            }}
        />

        <form
            onSubmit={(e) => {
                e.preventDefault();
                e.stopPropagation();
                form.handleSubmit();
            }}
        >
            <Stack style={{ width: '100%' }}>
                <Stack>
                    <Title order={2}>General</Title>
                    <form.Field
                        name="name"
                        validators={{
                            onBlur: ({ value }) => {
                                return (value.trimStart().length === 0 ? 'The field is required' : undefined) ||
                                    (value.length > 100 ? 'Maximum allowed chars is 100' : undefined)
                            }
                        }}
                        children={(field) => {
                            return <>
                                <TextInput
                                    data-1p-ignore
                                    data-cy="name"
                                    label="Name"
                                    description="Name of the new structure group"
                                    placeholder="My cool structure group"
                                    id={field.name}
                                    name={field.name}
                                    value={field.state.value}
                                    error={
                                        !field.state.meta.isValid && field.state.meta.errors.join(', ')
                                    }
                                    onBlur={field.handleBlur}
                                    onChange={(e) => field.handleChange(e.target.value)}
                                />
                            </>
                        }}
                    />

                    <Flex
                        justify='space-between'
                    >
                        <Title order={3}>Structures</Title>

                        <Button
                            onClick={openStructureSelector}
                        >
                            Edit structures
                        </Button>
                    </Flex>

                    <form.Field
                        name="structures"
                        children={(_) => {
                            return <>
                                <StructureSelectorModal
                                    opened={structureSelectorOpened}
                                    onClose={closeStructureSelector}
                                    onSelect={onSelectStructure}

                                    structures={structures}
                                    selected={selectedStructures}
                                />

                                <StructureList
                                    structures={selectedStructures}

                                    groupBySystem={false}
                                    viewTarget='_blank'
                                />
                            </>
                        }}
                    />

                    <Title order={3}>Layout</Title>
                    { structureLayouts() }

                    <Flex
                        justify='space-between'
                    >
                        <Title order={3}>Sharing</Title>

                        <Button
                            onClick={openEntitySelector}
                        >
                            Edit Shares
                        </Button>
                    </Flex>

                    <Alert color="gray">
                        Characters in the list, in the corporation or in the alliance listed below, will be able to see the industry hub, and save it into their account.
                    </Alert>

                    <CharacterCorporationAllianceList
                        characterCorporationAlliances={selectedEntities}
                    />
                </Stack>

                <Flex
                    justify="flex-end"
                    gap="sm"
                >
                    <form.Subscribe
                        selector={(state) => [state.canSubmit, state.isSubmitting]}
                        children={([canSubmit, isSubmitting]) => (
                            <Flex
                                justify="flex-end"
                                gap="sm"
                            >
                                <Button
                                    data-cy="saveStructure"
                                    mt="sm"
                                    type="submit"
                                    disabled={!canSubmit || isSubmitting}
                                    loading={isSubmitting}
                                >
                                    Save
                                </Button>
                            </Flex>
                        )}
                    />
                </Flex>

                { dangerZone() }
            </Stack>
        </form>
    </>;
}

export type TmpSystem = {
    systemId: number;
    systemName: string;
}
