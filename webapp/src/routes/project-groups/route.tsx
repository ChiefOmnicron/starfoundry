import { Filter, type FilterPropEntry, type SelectedFilter } from '@/components/Filter';
import { fetchProjectGroup, type ProjectGroup } from '@/services/project-group/fetch';
import { LIST_PROJECT_GROUPS, listProjectGroups } from '@/services/project-group/list';
import { Card, Table, Text, Title, UnstyledButton } from '@mantine/core';
import { useQueries, useQuery, type UseQueryResult } from '@tanstack/react-query';
import { createFileRoute, Link } from '@tanstack/react-router';
import { createColumnHelper, flexRender, getCoreRowModel, useReactTable } from '@tanstack/react-table';

export const Route = createFileRoute('/project-groups')({
    component: ProjectGroups,
});

const columnHelper = createColumnHelper<ProjectGroup>();
const columns = [
    columnHelper.accessor('name', {
        id: 'name',
        cell: info => <UnstyledButton
            component={Link}
            to={`/project-groups/${info.row.original.id}`}
            style={{
                color: 'var(--mantine-color-blue-4)'
            }}
        >
            { info.getValue() }
        </UnstyledButton>,
        header: () => 'Name',
    }),
    columnHelper.accessor('description', {
        id: 'description',
        cell: info => info.getValue(),
        header: () => 'Description',
    }),
    columnHelper.accessor('members', {
        id: 'members',
        cell: info => info.getValue(),
        header: () => 'Members',
    }),
    columnHelper.accessor('projects', {
        id: 'projects',
        cell: info => info.getValue(),
        header: () => 'Projects',
    }),
];

function ProjectGroups() {
    const {
        isPending,
        error,
        data: projectGroupUuids
    } = useQuery({
        queryKey: [LIST_PROJECT_GROUPS],
        queryFn: async () => listProjectGroups({}),
    });

    const projectGroups: UseQueryResult<ProjectGroup>[] = useQueries({
        queries: projectGroupUuids
            ? projectGroupUuids.map(uuid => {
                return {
                    queryKey: [LIST_PROJECT_GROUPS, uuid],
                    queryFn: async () => fetchProjectGroup(uuid)
                }
            })
            : []
    });

    const table = useReactTable<ProjectGroup>({
        columns: columns,
        data: projectGroups
            .filter(x => !!x.data)
            .map(x => x.data),
        getCoreRowModel: getCoreRowModel(),
    });

    if (isPending || projectGroups.filter(x => x.isPending).length > 0) {
        return 'Loading'
    }

    if (error) {
        return `Error: ${error.message}`;
    }

    if (projectGroups.filter(x => x.isError).length > 0) {
        return `Error: ${projectGroups.map(x => x.error)}`;
    }

    const exampleData: FilterPropEntry[] = [{
        label: 'Single Select',
        key: 'single',
        type: 'SELECT',
        options: [{
            label: 'A',
            key: 'a',
        }, {
            label: 'B',
            key: 'b',
        }, {
            label: 'C',
            key: 'c',
        }],
    }, {
        label: 'Multiselect',
        key: 'multi',
        type: 'MULTISELECT',
        options: [{
            label: 'D',
            key: 'd'
        }, {
            label: 'E',
            key: 'e'
        }, {
            label: 'F',
            key: 'f'
        }, {
            label: 'G',
            key: 'g'
        }]
    }, {
        label: 'Name',
        key: 'name',
        type: 'INPUT',
    }];

    const filterChange = (filters: SelectedFilter[]) => {
        console.log(filters)
    }

    return <>
        <Title order={1}>Project Groups</Title>
        <Text size='md'>Project groups allow you to set defaults and collaborate with other capsuleers</Text>

        <Filter
            entries={exampleData}
            onFilterChange={filterChange}
        />

        <Card
            padding='0'
            mt='md'
        >
            <div className='p-2'>
                <Table striped>
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
            </div>
        </Card>
    </>
}
