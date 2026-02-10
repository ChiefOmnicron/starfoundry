import { Alert, Button, Grid, Group, Stack, Table, Title } from '@mantine/core';
import { compareArray, SaveDialog } from '@/components/SaveDialog';
import { CopyText } from '@/components/CopyText';
import { createFileRoute, useNavigate } from '@tanstack/react-router'
import { DeleteResource } from '@/components/DeleteResource';
import { deleteStructure } from '@/services/structure/delete';
import { Dotlan } from '@/components/Dotlan';
import { FETCH_STRUCTURE, useFetchStructure } from '@/services/structure/fetch';
import { LIST_STRUCTURE, type StructureTax } from '@/services/structure/list';
import { LoadingAnimation } from '@/components/LoadingAnimation';
import { LoadingError } from '@/components/LoadingError';
import { RigSelector } from '@/components/selectors/RigSelector';
import { Route as StructureListRoute } from '@/routes/structures/index';
import { ServiceSelector } from '@/components/selectors/ServiceSelector';
import { systemRigBonusModifier } from '@/services/structure/utils';
import { TaxByService } from '@/routes/structures_/-components/TaxesByService';
import { updateStructure, type UpdateStructure } from '@/services/structure/update';
import { useEffect, useState } from 'react';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import type { TypeId } from '@/services/utils';
import { useDisclosure } from '@mantine/hooks';
import { StructureScanModal } from './-components/StructureScanModal';
import type { Item } from '@/services/item/model';

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
    const [taxes, setTaxes] = useState<StructureTax>({});

    const [touchedRigs, setTouchedRigs] = useState<boolean>(false);
    const [touchedServices, setTouchedServices] = useState<boolean>(false);
    const [touchedTaxes, setTouchedTaxes] = useState<boolean>(false);

    const [structureScanOpened, { open: openStructureScan, close: closeStructureScan }] = useDisclosure(false);

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
            setTaxes({ ...structure.taxes });
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

    useEffect(() => {
        setTouchedTaxes(JSON.stringify(taxes) !== JSON.stringify(structure?.taxes));
    }, [taxes]);

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
        setTaxes({ ...structure.taxes });
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

        <StructureScanModal
            opened={structureScanOpened}
            onClose={closeStructureScan}
            onParsed={(items: Item[]) => {
                // medium engineering rigs
                items
                    .filter(x => [
                        1816, 1817, 1818, 1819, 1820, 1821, 1822, 1823, 1824,
                        1825, 1826, 1827, 1828, 1829, 1830, 1831, 1832, 1833,
                        1834, 1835, 1836, 1837, 1838, 1839, 1840, 1841, 1842,
                        1843, 1844, 1845, 1846, 1847, 1848, 1849,
                    ].indexOf(x.group.group_id) > -1)
                    .map(x => setSelectedServices([...selectedServices, x.type_id]));
                // large engineering rigs
                items
                    .filter(x => [
                        1850, 1851, 1852, 1853, 1854, 1855, 1856, 1857, 1858,
                        1859, 1860, 1861, 1862, 1863, 1864, 1865, 1866,
                    ].indexOf(x.group.group_id) > -1)
                    .map(x => setSelectedServices([...selectedServices, x.type_id]));
                // x-large engineering rigs
                items
                    .filter(x => [
                        1867, 1868, 1869, 1870,
                    ].indexOf(x.group.group_id) > -1)
                    .map(x => setSelectedServices([...selectedServices, x.type_id]));
                // medium resource processing rigs
                items
                    .filter(x => [
                        1933, 1934, 1935, 1936, 1937, 1938, 1941, 1942, 1943,
                    ].indexOf(x.group.group_id) > -1)
                    .map(x => setSelectedServices([...selectedServices, x.type_id]));
                // large resource processing rigs
                items
                    .filter(x => [
                        1939, 1944,
                    ].indexOf(x.group.group_id) > -1)
                    .map(x => setSelectedServices([...selectedServices, x.type_id]));

                // citadel
                items
                    .filter(x => x.group.group_id === 1321)
                    .map(x => setSelectedServices([...selectedServices, x.type_id]));
                // resource processing
                items
                    .filter(x => x.group.group_id === 1322)
                    .map(x => setSelectedServices([...selectedServices, x.type_id]));
                // engineering
                items
                    .filter(x => x.group.group_id === 1415)
                    .map(x => setSelectedServices([...selectedServices, x.type_id]));
            }}
        />

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

                        <Group
                            justify='space-between'
                        >
                            <Title order={2}>Installed Rigs and Services</Title>

                            <Button
                                onClick={openStructureScan}
                            >
                                Set with Structure Scan
                            </Button>
                        </Group>
                        <RigSelector
                            rigs={structure.installable_rigs || []}
                            selected={selectedRigs}
                            onSelect={setSelectedRigs}
                            readonly={isReadonly()}
                        />

                        <ServiceSelector
                            services={structure.installable_services || { slots: 3, services: [] }}
                            selected={selectedServices}
                            onSelect={setSelectedServices}
                            readonly={isReadonly()}
                        />

                        <Title order={2}>Taxes</Title>
                        <TaxByService
                            taxes={taxes}
                            services={selectedServices}

                            onChange={(x) => {
                                setTaxes({
                                    ...x
                                })
                            }}
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
                    taxes:    taxes,
                });
            }}
            show={ touchedRigs || touchedServices || touchedTaxes }
        />
    </>;
}
