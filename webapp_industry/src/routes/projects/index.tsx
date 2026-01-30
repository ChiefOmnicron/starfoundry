import { Alert, Button, Center, Flex, Stack, Title } from '@mantine/core';
import { createFileRoute } from '@tanstack/react-router';
import { Filter, type FilterPropEntry, type SelectedFilter } from '@/components/Filter';
import { LoadingAnimation } from '@/components/LoadingAnimation';
import { LoadingError } from '@/components/LoadingError';
import { ProjectList } from '@/components/ProjectCard';
import { useDisclosure, useIsFirstRender } from '@mantine/hooks';
import { useListProjects, type ProjectFilter } from '@/services/projects/list';
import { useState } from 'react';

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
        label: 'Created',
        key: 'CREATED'
    }, {
        label: 'Initialized',
        key: 'INITIALIZED'
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

function RouteComponent() {
    const { deleted: deletedResource } = Route.useSearch();
    const [_, { open }] = useDisclosure(false);

    const isFirstRender = useIsFirstRender();

    // TODO: fix filtering
    const [filterParams, setFilterParams] = useState<ProjectFilter>({});
    const [selectedFilters, setSelectedFilters] = useState<SelectedFilter[]>([{
        filterKey: 'status',
        filterLabel: 'Status',
        key: 'IN_PROGRESS',
        value: 'In Progress',
    }]);

    const filterChange = (filters: SelectedFilter[]) => {
        setSelectedFilters(filters);
        console.log(filters, filters.find(x => x.key === 'name')?.value)
        setFilterParams({
            name: filters.find(x => x.filterKey === 'name')?.value as string,
            status: filters.find(x => x.filterKey === 'status')?.key as string,
        });
    };

    const {
        isPending,
        isError,
        isFetching,
        data: projects
    } = useListProjects(filterParams);

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

    const content = () => {
        if (isPending || isFetching) {
            return LoadingAnimation();
        } else if (isError) {
            return LoadingError();
        } else if (projects.length > 0) {
            return <>
                <ProjectList
                    projects={ projects }
                />
            </>;
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

        {
            isFirstRender
            ?   <></>
            :   <>
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
                        selectedFilter={selectedFilters}
                    />
                </>
        }

        { content() }
    </>
}
