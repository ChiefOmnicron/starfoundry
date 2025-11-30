import { Alert, Button, Card, Center, Flex, Modal, Stack, Table, Title } from '@mantine/core';
import { createColumnHelper, flexRender, getCoreRowModel, useReactTable } from '@tanstack/react-table';
import { createFileRoute } from '@tanstack/react-router';
import { Filter, type FilterPropEntry, type SelectedFilter } from '@/components/Filter';
import { LoadingError } from '@/components/LoadingError';
import { useEffect, useState } from 'react';
import { type StructureFilter, type System } from '@/services/structure/list';
import { Dotlan } from '@/components/Dotlan';
import { useDisclosure } from '@mantine/hooks';
import { Route as StructureGroupRoute } from '@/routes/structure-groups_/$structureGroupId.index';
import { InternalLink } from '@/components/InternalLink';
import { LoadingAnimation } from '@/components/LoadingAnimation';
import { useListStructureGroup, type StructureGroup } from '@/services/structure-group/list';
import type { Item } from '@/services/item/model';
import { normalizeRigServiceName } from '@/services/structure/utils';
import { AddStructureGroup } from './-modal/add';

interface QueryParams {
    deleted?: boolean;
}

export const Route = createFileRoute('/structure-groups/')({
    beforeLoad: async ({ context }) => {
        if (!(await context.auth.isAuthenticated())) {
            throw context.auth.login();
        }
    },
    component: ProjectGroups,
    validateSearch: (params: {
        deleted: boolean,
    }): QueryParams => {
        return {
            deleted: (params.deleted) || undefined
        };
    }
});

const columnHelper = createColumnHelper<StructureGroup>();
const columns = [
    columnHelper.accessor('name', {
        id: 'name',
        cell: info => <InternalLink
                to={ StructureGroupRoute.to }
                params={{
                    structureGroupId: info.row.original.id,
                }}
                content={ info.getValue() }
            />,
        header: () => 'Name',
    }),
    columnHelper.display({
        id: 'system',
        cell: info => {
            const systems: System[] = [];
            info
                .cell
                .row
                .original
                .structures
                .map(x => x.system)
                .forEach(x => {
                    const exists = systems.find(y => y.system_id === x.system_id);

                    if (!exists) {
                        systems.push(x);
                    }
                });

            return systems.map(x => <><Dotlan system={x} /><br /></>)
        },
        header: () => 'Systems',
    }),
    columnHelper.display({
        id: 'structures',
        cell: info => {
            const structures: Item[] = [];
            info
                .cell
                .row
                .original
                .structures
                .map(x => x.item)
                .forEach(x => {
                    const exists = structures.find(y => y.type_id === x.type_id);

                    if (!exists) {
                        structures.push(x);
                    }
                });

            return structures
                .sort((a, b) => a.name.localeCompare(b.name))
                .map(x => <>{ x.name }<br /></>)
        },
        header: () => 'Structures',
    }),
    columnHelper.display({
        id: 'services',
        cell: info => {
            const services: Item[] = [];
            info
                .cell
                .row
                .original
                .structures
                .flatMap(x => x.services)
                .forEach(x => {
                    const exists = services.find(y => y.type_id === x.type_id);

                    if (!exists) {
                        services.push(x);
                    }
                });

            return services
                .sort((a, b) => a.name.localeCompare(b.name))
                .map(x => <>{ normalizeRigServiceName(x.name) }<br /></>)
        },
        header: () => 'Services',
    }),
    columnHelper.display({
        id: 'rigs',
        cell: info => {
            const rigs: Item[] = [];
            info
                .cell
                .row
                .original
                .structures
                .flatMap(x => x.rigs)
                .map(x => x.item)
                .forEach(x => {
                    const exists = rigs.find(y => y.type_id === x.type_id);

                    if (!exists) {
                        rigs.push(x);
                    }
                });

            return rigs
                .sort((a, b) => a.name.localeCompare(b.name))
                .map(x => <>{ normalizeRigServiceName(x.name) }<br /></>)
        },
        header: () => 'Rigs',
    }),
];

function ProjectGroups() {
    const [opened, { open, close }] = useDisclosure(false);
    const { deleted: deletedResource } = Route.useSearch();

    const [filterParams, setFilterParams] = useState<StructureFilter | undefined>();
    const [filterOptions, setFilterOptions] = useState<FilterPropEntry[]>([]);

    const {
        isPending,
        isError,
        isSuccess,
        data: structureGroups,
    } = useListStructureGroup(filterParams || {});

    useEffect(() => {
        if (!isSuccess) {
            return;
        }

        if (filterParams) {
            return;
        }

        const uniqueStructures = new Map();
        (structureGroups || [])
            .flatMap(x => x.structures)
            .map(x => {
                return {
                    label: x.item.name,
                    key:   x.item.type_id,
                };
            })
            .map(x => uniqueStructures.set(x.key, x));

        const uniqueSystems = new Map();
        (structureGroups || [])
            .flatMap(x => x.structures)
            .map(x => {
                return {
                    label: x.system.system_name,
                    key:   x.system.system_id,
                };
            })
            .map(x => uniqueSystems.set(x.key, x));

        const uniqueServices = new Map();
        (structureGroups || [])
            .flatMap(x => x.structures)
            .map(x => x.services)
            .flatMap(x => {
                return x.map(y => {
                    return {
                        label: normalizeRigServiceName(y.name),
                        key:   y.type_id,
                    }
                });
            })
            .map(x => uniqueServices.set(x.key, x));

        const uniqueRigs = new Map();
        (structureGroups || [])
            .flatMap(x => x.structures)
            .map(x => x.rigs)
            .flatMap(x => {
                return x.map(y => {
                    return {
                        label: normalizeRigServiceName(y.item.name),
                        key:   y.item.type_id,
                    }
                });
            })
            .map(x => uniqueRigs.set(x.key, x));

        setFilterOptions([{
            label: 'Name',
            key: 'name',
            type: 'STRING',
        }, {
            label: 'Structure Type',
            key: 'structure_type_id',
            type: 'SELECT',
            options: [...uniqueStructures.values()].sort((a, b) => a.label.localeCompare(b.label)),
        }, {
            label: 'System',
            key: 'system_id',
            type: 'SELECT',
            options: [...uniqueSystems.values()].sort((a, b) => a.label.localeCompare(b.label)),
        }, {
            label: 'Service',
            key: 'service_id',
            type: 'SELECT',
            options: [...uniqueServices.values()].sort((a, b) => a.label.localeCompare(b.label)),
        }, {
            label: 'Rig',
            key: 'rig_id',
            type: 'SELECT',
            options: [...uniqueRigs.values()].sort((a, b) => a.label.localeCompare(b.label)),
        }]);
    }, [structureGroups]);

    const filterChange = (filters: SelectedFilter[]) => {
        setFilterParams({
            name: filters.find(x => x.filterKey === 'name')?.value as string,
            structure_type_id: filters.find(x => x.filterKey === 'structure_type_id')?.key as number,
            system_id: filters.find(x => x.filterKey === 'system_id')?.key as number,
            service_id: filters.find(x => x.filterKey === 'service_id')?.key as number,
            rig_id: filters.find(x => x.filterKey === 'rig_id')?.key as number,
        });
    };

    const table = useReactTable<StructureGroup>({
        columns: columns,
        data: structureGroups || [],
        autoResetPageIndex: false,
        getCoreRowModel: getCoreRowModel(),
    });

    const filter = <Filter
        entries={filterOptions}
        onFilterChange={filterChange}
    />;

    const addStructureModal = () => {
        return <Modal
            opened={ opened }
            onClose={ close }
            title="Add structure group"
            overlayProps={{
                backgroundOpacity: 0.55,
                blur: 3,
            }}
            size="70%"
            centered
            closeOnEscape
            closeOnClickOutside
        >
            <AddStructureGroup
                close={close}
            />
        </Modal>
    }

    const notification = () => {
        if (deletedResource) {
            return <Alert
                mt="sm"
                variant='light'
                color='green'
                title='Delete successful'
                data-cy="deleteSuccessful"
            >
                The structure group was successfully deleted
            </Alert>;
        }
    }

    const actionBar = () => {
        return <>
            <Flex
                align='center'
                justify='flex-start'
                direction='row-reverse'
                pb='sm'
            >
                <Button
                    variant='filled'
                    onClick={open}
                >
                    Add Structure Group
                </Button>
            </Flex>
        </>
    }

    const dataTable = () => {
        return <>
            { addStructureModal() }

            <Card p="0">
                <Table striped data-cy="data">
                    <Table.Thead>
                    {table.getHeaderGroups().map(headerGroup => (
                        <Table.Tr key={headerGroup.id}>
                            {headerGroup.headers.map(header => (
                                <Table.Th key={header.id}>
                                    {flexRender(
                                        header.column.columnDef.header,
                                        header.getContext()
                                    )}
                                </Table.Th>
                            ))}
                        </Table.Tr>
                    ))}
                    </Table.Thead>
                    <Table.Tbody>
                        {table.getRowModel().rows.map(row => (
                            <Table.Tr key={row.id}>
                                {row.getVisibleCells().map(cell => (
                                    <Table.Td key={cell.id}>
                                        {flexRender(cell.column.columnDef.cell, cell.getContext())}
                                    </Table.Td>
                                ))}
                            </Table.Tr>
                        ))}
                    </Table.Tbody>
                </Table>
            </Card>
        </>
    }

    const content = () => {
        if (isPending && !filterParams) {
            return LoadingAnimation();
        } else if (isError) {
            return LoadingError();
        } else if (isSuccess && structureGroups.length > 0) {
            return <>
                { actionBar() }

                { filter }

                { dataTable() }
            </>
        } else if (filterParams && isPending) {
            return <>
                { actionBar() }

                { filter }

                { LoadingAnimation() }
            </>
        } else if (filterParams && isSuccess && structureGroups.length === 0) {
            return <>
                { actionBar() }

                { filter }

                <Center mt={50} data-cy="noData">
                    <Title order={4}>No structure matching</Title>
                </Center>
            </>
        } else {
            return <>
                <Center mt={50} data-cy="noData">
                    <Stack>
                        <Title order={4}>No structures yet</Title>

                        <Button
                            variant='filled'
                            onClick={open}
                        >
                            Add Structure
                        </Button>
                    </Stack>
                </Center>
            </>
        }
    }

    return <>
        { notification() }

        { content() }
    </>
}
