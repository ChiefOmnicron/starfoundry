import { Alert, Button, Card, Center, Flex, Modal, Stack, Table, Title } from '@mantine/core';
import { createColumnHelper, flexRender, getCoreRowModel, useReactTable } from '@tanstack/react-table';
import { createFileRoute } from '@tanstack/react-router';
import { Filter, type FilterPropEntry, type SelectedFilter } from '@/components/Filter';
import { LoadingError } from '@/components/LoadingError';
import { useState } from 'react';
import LoadingAnimation from '@/components/LoadingAnimation';
import { useListStructure, type Structure, type StructureFilter } from '@/services/structure/list';
import { Dotlan } from '@/components/Dotlan';
import { AddStructure } from './-modal/add';
import { useDisclosure } from '@mantine/hooks';
import { Route as StructureRoute } from '@/routes/structures_/$structureId.index';
import { InternalLink } from '@/components/InternalLink';

interface QueryParams {
    deleted?: boolean;
}

export const Route = createFileRoute('/structures/')({
    beforeLoad: async ({ context }) => {
        if (!await context.auth.isAuthenticated()) {
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

const columnHelper = createColumnHelper<Structure>();
const columns = [
    columnHelper.accessor('name', {
        id: 'name',
        cell: info => <InternalLink
                to={ StructureRoute.to }
                params={{
                    structureId: info.row.original.id,
                }}
                content={ info.getValue() }
            />,
        header: () => 'Name',
    }),
    columnHelper.accessor('system', {
        id: 'system',
        cell: info => <Dotlan system={info.getValue()} />,
        header: () => 'System',
    }),
    columnHelper.accessor('item', {
        id: 'structure',
        cell: info => { return info.getValue().name },
        header: () => 'Type',
    }),
    columnHelper.accessor('services', {
        id: 'services',
        cell: info => { return <>{
            info
                .getValue()
                .map(x => {
                    let name = x.name.replace('Standup ', '');
                    return <label key={x.type_id}>{ name }</label>
                })
                .map(x => <>{x} <br /></>)
        }</> },
        header: () => 'Services',
    }),
    columnHelper.accessor('rigs', {
        id: 'rigs',
        cell: info => { return <>{
            info
                .getValue()
                .map(x => {
                    let name = x.item.name
                        .replace('Standup M-Set ', '')
                        .replace('Standup L-Set ', '')
                        .replace('Standup XL-Set ', '');

                    return <label key={x.item.type_id}>{ name }</label>
                })
                .map(x => <>{x} <br /></>)
        }</> },
        header: () => 'Services',
    }),
];

function ProjectGroups() {
    const [opened, { open, close }] = useDisclosure(false);
    const { deleted: deletedResource } = Route.useSearch();

    const [filterParams, setFilterParams] = useState<StructureFilter | undefined>();
    const filterOptions: FilterPropEntry[] = [{
        label: 'Name',
        key: 'name',
        type: 'STRING',
    }];
    const filterChange = (filters: SelectedFilter[]) => {
        setFilterParams({
            name: filters.find(x => x.key === 'name')?.value as string,
        });
    };

    const {
        isPending,
        isError,
        isSuccess,
        data: structures,
    } = useListStructure(filterParams || {});

    const table = useReactTable<Structure>({
        columns: columns,
        data: structures || [],
        autoResetPageIndex: false,
        getCoreRowModel: getCoreRowModel(),
    });

    const addStructureModal = () => {
        return <Modal
            opened={ opened }
            onClose={ close }
            title="Add structure"
            overlayProps={{
                backgroundOpacity: 0.55,
                blur: 3,
            }}
            size="70%"
            centered
            closeOnEscape
            closeOnClickOutside
        >
            <AddStructure
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
                The structure was successfully deleted
            </Alert>;
        }
    }

    const filter = () => {
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
                    Add Structure
                </Button>
            </Flex>

            <Filter
                entries={filterOptions}
                onFilterChange={filterChange}
            />
        </>
    }

    const dataTable = () => {
        return <>
            { addStructureModal() }

            { filter() }

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
        } else if (isSuccess && structures.length > 0) {
            return dataTable();
        } else if (filterParams && isSuccess && structures.length === 0) {
            return <>
                { filter() }

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
