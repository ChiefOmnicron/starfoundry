import { AddProjectGroup } from '@/routes/project-groups/-modal/add';
import { Alert, Button, Card, Center, Flex, Modal, Pill, Stack, Table, Tabs, Title } from '@mantine/core';
import { createColumnHelper, flexRender, getCoreRowModel, useReactTable } from '@tanstack/react-table';
import { createFileRoute } from '@tanstack/react-router';
import { Filter, type FilterPropEntry, type SelectedFilter } from '@starfoundry/components/misc/Filter';
import { Route as ProjectGroupRoute } from '@/routes/project-groups_/$projectGroupId.overview';
import { useDisclosure } from '@mantine/hooks';
import { useState } from 'react';
import type { ProjectGroup } from '@starfoundry/components/services/project-group/fetch';
import { InternalLink } from '@starfoundry/components/links/InternalLink';
import { useListProjectGroup, type ProjectGroupFilter } from '@starfoundry/components/services/project-group/list';
import { LoadingAnimation } from '@starfoundry/components/misc/LoadingAnimation';
import { LoadingError } from '@starfoundry/components/misc/LoadingError';

export interface QueryParams {
    deleted?: boolean;
}

export const Route = createFileRoute('/project-groups/')({
    beforeLoad: async ({ context }) => {
        if (!await context.auth.isAuthenticated()) {
            throw context.auth.login();
        }
    },
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
}];

const columnHelper = createColumnHelper<ProjectGroup>();
const columns = [
    columnHelper.accessor('name', {
        id: 'name',
        cell: info => <InternalLink
                to={ ProjectGroupRoute.to }
                params={{
                    projectGroupId: info.row.original.id,
                } as any}
                content={ info.getValue() }
            />,
        header: () => 'Name',
    }),
    columnHelper.accessor('description', {
        id: 'description',
        cell: info => info.getValue(),
        header: () => 'Description',
    }),
    columnHelper.accessor('is_owner', {
        id: 'is_owner',
        cell: info => {
            if (info.getValue()) {
                return <Pill>Owner</Pill>
            }
        },
        header: () => '',
    }),
    columnHelper.accessor('members', {
        id: 'members',
        cell: info => info.cell.row.original.members.length,
        header: () => 'Members',
    }),
    columnHelper.accessor('project_count', {
        id: 'project_count',
        cell: info => info.getValue(),
        header: () => 'Projects',
    }),
];

function RouteComponent() {
    const { deleted: deletedResource } = Route.useSearch();
    const [opened, { open, close }] = useDisclosure(false);

    const [archived, setArchived] = useState(false);

    const [filterParams, setFilterParams] = useState<ProjectGroupFilter>({});

    const filterChange = (filters: SelectedFilter[]) => {
        setFilterParams({
            name: filters.find(x => x.key === 'name')?.value as string,
        });
    };

    const {
        isPending,
        isError,
        isFetching,
        data: projectGroups
    } = useListProjectGroup({
        archived,
        ...filterParams,
    });

    const table = useReactTable<ProjectGroup>({
        columns: columns,
        data: projectGroups,
        autoResetPageIndex: false,
        getCoreRowModel: getCoreRowModel(),
    });

    const onTabChange = (tab: string | null) => {
        if (tab === 'archived') {
            setArchived(true);
        } else {
            setArchived(false);
        }
    }

    const addProjectGroup = () => {
        return <Modal
            opened={ opened }
            onClose={ close }
            title="Add project group"
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

    const actionBar = () => {
        return <Flex
            align='center'
            justify='flex-start'
            direction='row-reverse'
            pb='sm'
        >
            <Button
                variant='filled'
                onClick={ open }
            >
                Create Group
            </Button>
        </Flex>
    }

    const dataTable = () => {
        return <>
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
        } else if (projectGroups.length > 0) {
            return dataTable();
        } else if (projectGroups.length === 0 && !archived) {
            return <>
                <Center mt={50} data-cy="noData">
                    <Stack>
                        <Title order={4}>No project groups yet</Title>

                        <Button
                            variant='filled'
                            onClick={ open }
                        >
                            Create Group
                        </Button>
                    </Stack>
                </Center>
            </>
        } else if (projectGroups.length === 0 && archived) {
            return <>
                <Center mt={50} data-cy="noData">
                    <Stack>
                        <Title order={4}>No archived project groups</Title>
                    </Stack>
                </Center>
            </>
        }
    }

    return <>
        { notification() }

        { addProjectGroup() }

        { actionBar() }

        <Tabs
            defaultValue="active"
            onChange={onTabChange}
        >
            <Tabs.List>
                <Tabs.Tab value="active">
                    Active
                </Tabs.Tab>

                <Tabs.Tab value="archived">
                    Archived
                </Tabs.Tab>
            </Tabs.List>

            <Tabs.Panel value="active">
                { content() }
            </Tabs.Panel>

            <Tabs.Panel value="archived">
                { content() }
            </Tabs.Panel>
        </Tabs>
    </>
}
