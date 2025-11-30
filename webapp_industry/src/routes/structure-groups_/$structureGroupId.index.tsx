import { Alert, Button, Flex, Grid, Stack, TextInput, Title } from '@mantine/core';
import { createFileRoute, useNavigate } from '@tanstack/react-router'
import { DeleteResource } from '@/components/DeleteResource';
import { deleteStructureGroup } from '@/services/structure-group/delete';
import { FETCH_STRUCTURE_GROUP } from '@/services/structure-group/fetch';
import { LIST_STRUCTURE_GROUP } from '@/services/structure-group/list';
import { useListStructure, type Structure } from '@/services/structure/list';
import { LoadingAnimation } from '@/components/LoadingAnimation';
import { LoadingError } from '@/components/LoadingError';
import { Route as StructureListRoute } from '@/routes/structures/index';
import { useFetchStructureGroup } from '@/services/structure-group/fetch';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { useEffect, useState } from 'react';
import { StructureLayout } from '@/components/StructureLayout';
import { useForm } from '@tanstack/react-form';
import { updateStructureGroup, type UpdateStructureGroup } from '@/services/structure-group/update';
import { StructureList } from '@/components/StructureList';

interface QueryParams {
    created?: boolean;
}

export const Route = createFileRoute('/structure-groups_/$structureGroupId/')({
    component: StructureComponent,
    validateSearch: (params: {
        created: boolean,
    }): QueryParams => {
        return {
            created: (params.created) || undefined
        };
    }
})

function StructureComponent() {
    const { structureGroupId } = Route.useParams();
    const { created: createdResource } = Route.useSearch();

    const navigation = useNavigate();
    const queryClient = useQueryClient();

    const [successfulUpdate, setSuccessfulUpdate] = useState<boolean>();
    const [successfulDelete, setSuccessfulDelete] = useState<boolean>();
    const [errorDelete, setErrorDelete] = useState<string | undefined>();
    const [errorUpdate, setErrorUpdate] = useState<string | undefined>();

    const [selectedStructures, setSelectedStructures] = useState<Structure[]>([]);

    const {
        isPending,
        isError,
        data: structureGroup,
    } = useFetchStructureGroup(structureGroupId);

    const {
        isPending: isPendingStructures,
        isError: isErrorStructures,
        data: structures,
    } = useListStructure({});

    const mutationUpdate = useMutation({
        mutationFn: (data: UpdateStructureGroup) => updateStructureGroup(structureGroupId, data),
        onSuccess: () => {
            setSuccessfulUpdate(true);
            queryClient.invalidateQueries({ queryKey: [FETCH_STRUCTURE_GROUP, structureGroupId] });
            queryClient.invalidateQueries({ queryKey: [LIST_STRUCTURE_GROUP] });
        },
    });

    const mutationDelete = useMutation({
        mutationFn: () => deleteStructureGroup(structureGroupId),
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: [LIST_STRUCTURE_GROUP] });
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
        setSelectedStructures(structureGroup.structures);
    }, [structureGroup]);

    const form = useForm({
        defaultValues: {
            name:       structureGroup.name,
            structures: structureGroup.structures.map(x => x.id),
        },
        onSubmit: async ({ value }) => {
            console.log(value)
            return await mutationUpdate
                .mutateAsync({
                    ...value,
                    structures: selectedStructures.map(x => x.id),
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

    const bonuses = () => {
        return selectedStructures
            .map(structure => {
                let systemModifier = 1;
                switch(structure.system.security_str){
                    case 'LOWSEC':
                        systemModifier = 1.9;
                        break
                    case 'NULLSEC':
                        systemModifier = 2.1;
                        break
                    default:
                        systemModifier = 1
                        break;
                }

                return structure
                    .rigs
                    .map(x => {
                        const bonus = x.categories.length > 0
                            ? x.categories.map(x => x.name).join(', ')
                            : x.groups.map(x => x.name).join(', ');

                        if (x.material && x.time) {
                            return <>
                                <label>-{ (x.material * systemModifier).toFixed(2) }% ME bonus for '{ bonus }'</label><br />
                                <label>-{ (x.time * systemModifier).toFixed(2) }% TE bonus for '{ bonus }'</label><br />
                            </>
                        } else if (x.material) {
                            return <>
                                <label>-{ (x.material * systemModifier).toFixed(2) }% ME bonus for '{ bonus }'</label><br />
                            </>
                        } else if (x.time) {
                            return <>
                                <label>-{ (x.time * systemModifier).toFixed(2) }% TE bonus for '{ bonus }'</label><br />
                            </>
                        }
                    });
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
                resource={ structureGroup.name }
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

    const removeSelectedStructure = (structureId: string) => {
        const removedStructure = selectedStructures
            .filter(x => x.id !== structureId);
        setSelectedStructures(removedStructure)
    }

    const addSelectedStructure = (structure: Structure) => {
        setSelectedStructures([
            structure,
            ...selectedStructures,
        ]);
    }

    return <>
        { notification() }

        <form
            onSubmit={(e) => {
                e.preventDefault();
                e.stopPropagation();
                form.handleSubmit();
            }}
        >
            <Stack style={{ width: '100%' }}>
                <Grid>
                    <Grid.Col span={{ base: 12, sm: 7}}>
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

                            <Title order={2}>Structures</Title>
                            <form.Field
                                name="structures"
                                children={(_) => {
                                    return <>
                                        <StructureList
                                            structures={selectedStructures}
                                            selectableStructures={structures}
                                            onDelete={removeSelectedStructure}
                                            onSelect={addSelectedStructure}
                                        />
                                    </>
                                }}
                            />

                            <Title order={3}>Layout</Title>
                            { structureLayouts() }
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
                    </Grid.Col>

                    <Grid.Col span={{ base: 12, sm: 5}}>
                        <Title order={2}>Bonuses</Title>

                        { bonuses() }
                    </Grid.Col>
                </Grid>
            </Stack>
        </form>
    </>;
}

export type TmpSystem = {
    systemId: number;
    systemName: string;
}
