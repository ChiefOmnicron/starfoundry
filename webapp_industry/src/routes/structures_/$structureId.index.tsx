import { Alert, Grid, Stack, Table, Title } from '@mantine/core';
import { createFileRoute, useNavigate } from '@tanstack/react-router'
import { DeleteResource } from '@/components/DeleteResource';
import { deleteStructure } from '@/services/structure/delete';
import { Dotlan } from '@/components/Dotlan';
import { FETCH_STRUCTURE, useFetchStructure } from '@/services/structure/fetch';
import { LIST_STRUCTURE } from '@/services/structure/list';
import { LoadingAnimation } from '@/components/LoadingAnimation';
import { LoadingError } from '@/components/LoadingError';
import { RigSelector } from '@/components/selectors/RigSelector';
import { Route as StructureListRoute } from '@/routes/structures/index';
import { ServiceSelector } from '@/components/selectors/ServiceSelector';
import { updateStructure, type UpdateStructure } from '@/services/structure/update';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { useEffect, useState } from 'react';
import type { TypeId } from '@/services/utils';
import { compareArray, SaveDialog } from '@/components/SaveDialog';
import { CopyText } from '@/components/CopyText';
import { systemRigBonusModifier } from '@/services/structure/utils';

interface QueryParams {
    created?: boolean;
}

export const Route = createFileRoute('/structures_/$structureId/')({
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
    const { structureId } = Route.useParams();
    const { created: createdResource } = Route.useSearch();
    const navigation = useNavigate();
    const queryClient = useQueryClient();

    const [successfulUpdate, setSuccessfulUpdated] = useState<boolean>();
    const [errorDelete, setErrorDelete] = useState<string | undefined>();
    const [errorUpdate, setErrorUpdated] = useState<string | undefined>();

    const [selectedRigs, setSelectedRigs] = useState<TypeId[]>([]);
    const [selectedServices, setSelectedServices] = useState<(TypeId)[]>([]);

    const [touchedRigs, setTouchedRigs] = useState<boolean>(false);
    const [touchedServices, setTouchedServices] = useState<boolean>(false);

    const {
        isPending,
        isError,
        data: structure,
    } = useFetchStructure(structureId, {
        include_installable: true,
    });

    const mutationUpdate = useMutation({
        mutationFn: (data: UpdateStructure) => updateStructure(structureId, data),
        onSuccess: () => {
            setSuccessfulUpdated(true);
            queryClient.invalidateQueries({ queryKey: [FETCH_STRUCTURE, structureId] });
            queryClient.invalidateQueries({ queryKey: [LIST_STRUCTURE] });
        },
    });

    const mutationDelete = useMutation({
        mutationFn: () => deleteStructure(structureId),
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: [LIST_STRUCTURE] });

            navigation({
                to: StructureListRoute.to,
                search: {
                    deleted: true,
                }
            });
        },
    });

    useEffect(() => {
        if (structure) {
            setSelectedRigs(structure.rigs.map(x => x.item.type_id));
            setSelectedServices(structure.services.map(x => x.type_id));
        }
    }, [structure]);

    useEffect(() => {
        const a = (structure || { rigs: [] }).rigs.map(x => x.item.type_id);
        const b = selectedRigs;
        setTouchedRigs(!compareArray(a, b));
    }, [selectedRigs]);

    useEffect(() => {
        const a = (structure || { services: [] }).services.map(x => x.type_id);
        const b = selectedServices;
        setTouchedServices(!compareArray(a, b));
    }, [selectedServices]);

    if (isPending) {
        return LoadingAnimation();
    }

    if (isError) {
        return LoadingError();
    }

    const isReadonly = () => {
        // Jita
        return structure.structure_id === 60003760 ||
        // Amarr
            structure.structure_id === 60008494
    }

    const deleteResource = async () => {
        await mutationDelete
            .mutateAsync()
            .catch(error => {
                setErrorDelete(error);
                setSuccessfulUpdated(false);
            });
    }

    const resetChanges = () => {
        setSelectedRigs(structure.rigs.map(x => x.item.type_id));
        setSelectedServices(structure.services.map(x => x.type_id));
    }

    const bonuses = () => {
        let systemModifier = systemRigBonusModifier(structure.system.security_str);

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
            })
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
                onClose={ () => setSuccessfulUpdated(false) }
                withCloseButton
            >
                The structure was updated
            </Alert>;
        } else if (errorUpdate) {
            return <Alert
                mt="sm"
                variant='light'
                color='red'
                title='Update error'
                data-cy="errorUpdate"
                onClose={ () => setErrorUpdated(undefined) }
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
        if (isReadonly()) {
            return <></>
        }

        return <>
            <Title
                data-cy="danger-zone-header"
                order={2}
                mt="md"
            >
                Danger Zone
            </Title>

            <DeleteResource
                resource={ structure.name }
                onConfirm={ () => deleteResource() }
            />
        </>
    }

    return <>
        { notification() }

        TAXES
        WHAT CAN BE BUILD

        <Stack style={{ width: '100%' }}>
            <Grid>
                <Grid.Col span={{ base: 12, sm: 7}}>
                    <Stack>
                        <Title order={2}>Information</Title>

                        <Table>
                            <Table.Tbody>
                                <Table.Tr>
                                    <Table.Th>Name</Table.Th>
                                    <Table.Td>
                                        <CopyText
                                            value={structure.name}
                                        />
                                    </Table.Td>
                                </Table.Tr>
                                <Table.Tr>
                                    <Table.Th>In-Game ID</Table.Th>
                                    <Table.Td>
                                        <CopyText
                                            value={structure.structure_id}
                                        />
                                    </Table.Td>
                                </Table.Tr>
                                <Table.Tr>
                                    <Table.Th>Type</Table.Th>
                                    <Table.Td>
                                        <CopyText
                                            value={structure.item.name}
                                        />
                                    </Table.Td>
                                </Table.Tr>
                                <Table.Tr>
                                    <Table.Th>System</Table.Th>
                                    <Table.Td>
                                        <Dotlan
                                            system={structure.system}
                                        />
                                    </Table.Td>
                                </Table.Tr>
                            </Table.Tbody>
                        </Table>

                        <RigSelector
                            rigs={structure.installable_rigs || []}
                            selected={selectedRigs}
                            onSelect={(selected: TypeId[]) => {
                                setSelectedRigs(selected);
                            }}
                            readonly={isReadonly()}
                        />

                        <ServiceSelector
                            services={structure.installable_services || { slots: 3, services: [] }}
                            selected={selectedServices}
                            onSelect={(selected: TypeId[]) => {
                                setSelectedServices(selected);
                            }}
                            readonly={isReadonly()}
                        />
                    </Stack>

                    { dangerZone() }
                </Grid.Col>

                <Grid.Col span={{ base: 12, sm: 5}}>
                    <Title order={2}>Bonuses</Title>

                    { bonuses() }
                </Grid.Col>
            </Grid>
        </Stack>

        <SaveDialog
            onReset={resetChanges}
            onSave={() => {
                mutationUpdate.mutate({
                    rigs:     selectedRigs,
                    services: selectedServices,
                });
            }}
            show={ touchedRigs || touchedServices }
        />
    </>;
}
