import { AddProjectGroup } from '@/routes/project-groups/-modal/add';
import { Alert, Badge, Button, Card, Center, Flex, Modal, NumberFormatter, Stack, Table, Title } from '@mantine/core';
import { createColumnHelper, flexRender, getCoreRowModel, useReactTable } from '@tanstack/react-table';
import { createFileRoute } from '@tanstack/react-router';
import { Filter, type FilterPropEntry, type SelectedFilter } from '@/components/Filter';
import { LoadingAnimation } from '@/components/LoadingAnimation';
import { LoadingError } from '@/components/LoadingError';
import { Route as ProjectRoute } from '@/routes/projects_/$projectId.index';
import { useState } from 'react';
import { useDisclosure } from '@mantine/hooks';
import { useListProjects, type ProjectFilter, type ProjectList } from '@/services/projects/list';
import { InternalLink } from '@/components/InternalLink';
import { ProjectProgressBar } from '@/routes/projects/-components/ProgressBar';

interface QueryParams {
    deleted?: boolean;
}

export const Route = createFileRoute('/projects/')({
    component: RouteComponent,
    validateSearch: (params: {
        deleted: boolean,
    }): QueryParams => {
        return {
            deleted: (params.deleted) || undefined
        };
    }
});

const filters: FilterPropEntry[] = [{
    label: 'Name',
    key: 'name',
    type: 'STRING',
}, {
    label: 'Status',
    key: 'status',
    type: 'SELECT',
    options: [{
        label: 'Preparing',
        key: 'PREPARING'
    }, {
        label: 'In Progress',
        key: 'IN_PROGRESS'
    }, {
        label: 'Paused',
        key: 'PAUSED'
    }, {
        label: 'Done',
        key: 'DONE'
    }]
}];

const columnHelper = createColumnHelper<ProjectList>();
const columns = [
    columnHelper.accessor('name', {
        id: 'name',
        cell: info => <InternalLink
                to={ProjectRoute.to}
                params={{
                    projectId: info.row.original.id,
                } as any}
                content={ info.getValue() }
            />,
        header: () => 'Name',
    }),
    columnHelper.accessor('orderer', {
        id: 'orderer',
        cell: info => info.getValue(),
        header: () => 'Orderer',
    }),
    columnHelper.accessor('status', {
        id: 'status',
        cell: info => {
            switch (info.getValue()) {
                case 'DONE':
                    return <Badge color="green">Done</Badge>;
                case 'IN_PROGRESS':
                    return <Badge color="blue">In Progress</Badge>;
                case 'PAUSED':
                    return <Badge color="yellow">Pause</Badge>;
                default:
                    return <Badge color="gray">Initial</Badge>;
            }
        },
        header: () => 'Status',
        maxSize: 1,
    }),
    columnHelper.accessor('sell_price', {
        id: 'sell_price',
        cell: info => {
            if (info.getValue()) {
                return <NumberFormatter
                    value={ info.getValue() }
                    suffix=" ISK"
                    thousandSeparator
                />
            }
        },
        header: () => 'Sell price',
    }),
    columnHelper.display({
        id: 'progress',
        cell: info => <ProjectProgressBar projectId={info.row.original.id} />,
        header: () => 'Progress',
    })
];

export function RouteComponent() {
    const { deleted: deletedResource } = Route.useSearch();
    const [opened, { open, close }] = useDisclosure(false);

    // TODO: fix filtering
    const [filterParams, setFilterParams] = useState<ProjectFilter>({
        status: 'IN_PROGRESS',
    });
    const filterChange = (filters: SelectedFilter[]) => {
        setFilterParams({
            name: filters.find(x => x.key === 'name')?.value as string,
            status: filters.find(x => x.filterKey === 'status')?.key as string,
        });
    };

    const {
        isPending,
        isError,
        isFetching,
        data: projects
    //} = useListProjects(filterParams);
    } = useListProjects({
        status: 'IN_PROGRESS'
    });

    const table = useReactTable<ProjectList>({
        columns: columns,
        data: projects,
        autoResetPageIndex: false,
        getCoreRowModel: getCoreRowModel(),
    });

    const addProject = () => {
        return <Modal
            opened={ opened }
            onClose={ close }
            title="Add project"
            overlayProps={{
                backgroundOpacity: 0.55,
                blur: 3,
            }}
            size="70%"
            centered
            closeOnEscape
            closeOnClickOutside
        >
            <AddProjectGroup
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
                The project group was successfully deleted
            </Alert>;
        }
    }

    const dataTable = () => {
        return <>
            <Flex
                align='center'
                justify='flex-start'
                direction='row-reverse'
                pb='sm'
            >
                <Button
                    variant='filled'
                    onClick={ open }
                >
                    Create Project
                </Button>
            </Flex>

            <Filter
                entries={filters}
                onFilterChange={filterChange}
            />

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
        if (isPending || isFetching) {
            return LoadingAnimation();
        } else if (isError) {
            return LoadingError();
        } else if (projects.length > 0) {
            return dataTable();
        } else {
            return <>
                <Center mt={50} data-cy="noData">
                    <Stack>
                        <Title order={4}>No projects yet</Title>

                        <Button
                            variant='filled'
                            onClick={ open }
                        >
                            Create Project
                        </Button>
                    </Stack>
                </Center>
            </>
        }
    }

    return <>
        { notification() }

        { addProject() }

        { content() }
    </>
}
