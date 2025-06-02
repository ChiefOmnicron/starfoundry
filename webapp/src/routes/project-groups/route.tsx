import Filter, { type FilterPropEntry, type SelectedFilter } from '@/components/Filter';
import { FETCH_PROJECT_GROUPS, fetchProjectGroup, type ProjectGroup } from '@/services/project-group/fetch';
import { LIST_PROJECT_GROUPS, listProjectGroups, type ProjectGroupFilter } from '@/services/project-group/list';
import type { Uuid } from '@/services/utils';
import { Card, Table, Text, Title, UnstyledButton } from '@mantine/core';
import { useQueries, useQuery, useQueryClient, type UseQueryResult } from '@tanstack/react-query';
import { createFileRoute, Link } from '@tanstack/react-router';
import { createColumnHelper, flexRender, getCoreRowModel, useReactTable } from '@tanstack/react-table';
import { useCallback, useEffect, useMemo, useRef, useState } from 'react';

export const Route = createFileRoute('/project-groups')({
    component: ProjectGroups,
});

const filters: FilterPropEntry[] = [{
    label: 'Name',
    key: 'name',
    type: 'STRING',
}];

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
    const [filterParams, setFilterParams] = useState<ProjectGroupFilter>({});

    const filterChange = (filters: SelectedFilter[]) => {
        setFilterParams({
            name: filters.find(x => x.key === 'name')?.value as string,
        });
    };

    const {
        isPending,
        error,
        data: projectGroupUuids
    } = useQuery({
        queryKey: [LIST_PROJECT_GROUPS, filterParams],
        queryFn: async () => listProjectGroups(filterParams),
    });

    const projectGroups: UseQueryResult<ProjectGroup>[] = useQueries({
        queries: projectGroupUuids
            ? projectGroupUuids.map((uuid: Uuid) => {
                return {
                    queryKey: [FETCH_PROJECT_GROUPS, uuid],
                    queryFn: async () => fetchProjectGroup(uuid),
                }
            })
            : []
    });

    const table = useReactTable<ProjectGroup>({
        columns: columns,
        data: projectGroups
            .filter(x => !!x.data)
            .map(x => x.data),
        autoResetPageIndex: false,
        getCoreRowModel: getCoreRowModel(),
    });

    /*if (isPending || projectGroups.filter(x => x.isPending).length > 0) {
        return 'Loading'
    }*/

    if (error) {
        return `Error: ${error.message}`;
    }

    if (projectGroups.filter(x => x.isError).length > 0) {
        return `Error: ${projectGroups.map(x => x.error)}`;
    }

    return <>
        <Title order={1}>Project Groups</Title>
        <Text size='md'>Project groups allow you to set defaults and collaborate with other capsuleers</Text>

        <Filter
            entries={filters}
            onFilterChange={filterChange}
        />

        { projectGroups.map(x => x.data?.name) }
        { projectGroups.map(x => x.data?.members) }
        { projectGroups.map(x => x.data?.description) }
        { projectGroups.map(x => x.data?.projects) }

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
    </>
}
