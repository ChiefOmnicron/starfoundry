import { Alert, Button, Center, Flex, Stack, Tabs, Title } from '@mantine/core';
import { createFileRoute, useNavigate } from '@tanstack/react-router';
import { CreateProjectModal } from '@starfoundry/components/project/CreateProject';
import { LoadingAnimation } from '@starfoundry/components/misc/LoadingAnimation';
import { LoadingError } from '@starfoundry/components/misc/LoadingError';
import { ProjectList } from '@starfoundry/components/project/ProjectList';
import { Route as ProjectAssistantRoute } from '@/routes/projects_/$projectId.assistant/index';
import { Route as ProjectOverviewRoute } from '@/routes/projects_/$projectId.overview';
import { useDisclosure, useIsFirstRender } from '@mantine/hooks';
import { useEffect, useState } from 'react';
import { useListProjects, type ProjectFilter } from '@starfoundry/components/services/projects/list';
import { CompositeFiltersInput, useCompositeFilters, type ActiveFilter, type FilterDefinition } from 'mantine-composite-filters';
import type { Uuid } from '@starfoundry/components/services/utils';

export interface QueryParams {
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

const filterDefinitions: FilterDefinition[] = [{
    label: 'Name',
    key: 'name',
    type: 'text',
    placeholder: 'Search ...',
    operators: ['contains']
}, {
    label: 'Status',
    key: 'status',
    type: 'select',
    options: [{
        label: 'Draft',
        value: 'DRAFT'
    }, {
        label: 'Ready to start',
        value: 'READY_TO_START'
    }, {
        label: 'In Progress',
        value: 'IN_PROGRESS'
    }, {
        label: 'Paused',
        value: 'PAUSED'
    }, {
        label: 'Done',
        value: 'DONE'
    }]
}];

const INITIAL_FILTER: ActiveFilter = {
    displayValue: 'In Progress',
    id: 'status',
    key: 'status',
    label: 'Status',
    operator: '=',
    type: 'select',
    value: 'IN_PROGRESS',
}

function RouteComponent() {
    const {
        activeFilters,
        setActiveFilters,

        resetFilters,
        replaceFilter,
    } = useCompositeFilters({
        filterDefinitions,
        onFiltersChange: (filters) => onFilterChange(filters),
        initialFilters: [INITIAL_FILTER],
    });

    const navigation = useNavigate();
    const { deleted: deletedResource } = Route.useSearch();
    const [opened, { open, close }] = useDisclosure(false);

    const [activeTab, setActiveTab] = useState<string| null>('in_progress');

    const isFirstRender = useIsFirstRender();

    const [filterParams, setFilterParams] = useState<ProjectFilter>({
        status: 'IN_PROGRESS',
    });

    const onFilterChange = (filters: ActiveFilter[]) => {
        if (filters.length === 0) {
            resetFilters();
            filterByTab();
            return;
        }

        setFilterParams({
            name: filters.find(x => x.key === 'name')?.value as string,
            status: filters.find(x => x.key === 'status')?.value as string,
        });
    };

    useEffect(() => {
        filterByTab();
    }, [activeTab]);

    const filterByTab = () => {
        if (activeTab === 'in_progress') {
            replaceFilter('status', {
                key: 'status',
                displayValue: 'In Progress',
                label: 'Status',
                operator: '=',
                type: 'select',
                value: 'IN_PROGRESS'
            });
        } else if (activeTab === 'drafts') {
            replaceFilter('status', {
                key: 'status',
                displayValue: 'Drafts',
                label: 'Status',
                operator: '=',
                type: 'select',
                value: 'DRAFT'
            });
        } else if (activeTab === 'ready_to_start') {
            replaceFilter('status', {
                key: 'status',
                displayValue: 'Ready to Start',
                label: 'Status',
                operator: '=',
                type: 'select',
                value: 'READY_TO_START'
            });
        }
    }

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

    const tabContent = () => {
        if (isPending || isFetching) {
            return LoadingAnimation();
        } else if (isError) {
            return LoadingError();
        }

        if (projects.length === 0) {
            return <>
                <CompositeFiltersInput
                    filters={filterDefinitions}
                    value={activeFilters}
                    onChange={(filter) => {
                        setActiveFilters(filter);
                        onFilterChange(filter);
                    }}
                />

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

        return <>
            <CompositeFiltersInput
                filters={filterDefinitions}
                value={activeFilters}
                onChange={(filter) => {
                    setActiveFilters(filter);
                    onFilterChange(filter);
                }}
                variant='minimal'
                disablePresets
                disableHistory
            />

            <ProjectList
                projects={ projects }

                projectCardProps={{
                    viewLink: ProjectOverviewRoute.to,
                    assistantLink: ProjectAssistantRoute.to,
                }}
            />
        </>
    }

    const content = () => {
        return <>
            <Tabs
                value={activeTab}
                onChange={(tab) => {
                    setActiveTab(tab);

                    if (tab === 'in_progress') {
                        setFilterParams({
                            status: 'IN_PROGRESS',
                        });
                    } else if (tab === 'drafts') {
                        setFilterParams({
                            status: 'DRAFT',
                        });
                    } else if (tab === 'ready_to_start') {
                        setFilterParams({
                            status: 'READY_TO_START',
                        });
                    }
                }}
            >
                <Tabs.List>
                    <Tabs.Tab value="in_progress">
                        In Progress
                    </Tabs.Tab>
                    <Tabs.Tab value="ready_to_start">
                        Ready to Start
                    </Tabs.Tab>
                    <Tabs.Tab value="drafts">
                        Drafts
                    </Tabs.Tab>
                </Tabs.List>

                <Tabs.Panel value="in_progress">
                    { tabContent() }
                </Tabs.Panel>

                <Tabs.Panel value="drafts">
                    { tabContent() }
                </Tabs.Panel>

                <Tabs.Panel value="ready_to_start">
                    { tabContent() }
                </Tabs.Panel>
            </Tabs>
        </>;
    }

    return <>
        { notification() }

        <CreateProjectModal
            opened={opened}
            close={close}

            onCreate={(projectId: Uuid) => {
                return navigation({
                    to: ProjectAssistantRoute.to,
                    params: {
                        projectId: projectId,
                    },
                    search: {
                        created: true,
                    },
                });
            }}
        />

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
                </>
        }

        { content() }
    </>
}
