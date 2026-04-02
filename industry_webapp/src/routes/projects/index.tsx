import { Alert, Button, Center, Flex, Stack, Tabs, Title } from '@mantine/core';
import { createFileRoute, useNavigate } from '@tanstack/react-router';
import { CreateProjectModal } from '@starfoundry/components/project/CreateProject';
import { LoadingAnimation } from '@starfoundry/components/misc/LoadingAnimation';
import { LoadingError } from '@starfoundry/components/misc/LoadingError';
import { ProjectList } from '@starfoundry/components/project/ProjectList';
import { Route as ProjectAssistantRoute } from '@/routes/projects_/$projectId.assistant/index';
import { Route as ProjectOverviewRoute } from '@/routes/projects_/$projectId.overview';
import { useDisclosure, useIsFirstRender } from '@mantine/hooks';
import { useListProjects, type ProjectFilter } from '@starfoundry/components/services/projects/list';
import { useState } from 'react';
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

//const filters: FilterPropEntry[] = [{
//    label: 'Name',
//    key: 'name',
//    type: 'STRING',
//}, {
//    label: 'Status',
//    key: 'status',
//    type: 'SELECT',
//    options: [{
//        label: 'Draft',
//        key: 'DRAFT'
//    }, {
//        label: 'Ready to start',
//        key: 'READY_TO_START'
//    }, {
//        label: 'In Progress',
//        key: 'IN_PROGRESS'
//    }, {
//        label: 'Paused',
//        key: 'PAUSED'
//    }, {
//        label: 'Done',
//        key: 'DONE'
//    }]
//}];

function RouteComponent() {
    const navigation = useNavigate();
    const { deleted: deletedResource } = Route.useSearch();
    const [opened, { open, close }] = useDisclosure(false);

    const [activeTab, setActiveTab] = useState<string| null>('in_progress');

    const isFirstRender = useIsFirstRender();

    const [filterParams, setFilterParams] = useState<ProjectFilter>({
        status: 'IN_PROGRESS',
    });
    //const [selectedFilters, setSelectedFilters] = useState<SelectedFilter[]>([{
    //    filterKey: 'status',
    //    filterLabel: 'Status',
    //    key: 'IN_PROGRESS',
    //    value: 'In Progress',
    //}]);

    //const filterChange = (filters: SelectedFilter[]) => {
    //    if (filters.length === 0) {
    //        return;
    //    }
//
    //    setSelectedFilters(filters);
    //    setFilterParams({
    //        name: filters.find(x => x.filterKey === 'name')?.value as string,
    //        status: filters.find(x => x.filterKey === 'status')?.key as string,
    //    });
    //};

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
            {
                //<Filter
                //    entries={filters}
                //    onFilterChange={filterChange}
                //    selectedFilter={selectedFilters}
                ///>
            }

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
                        console.log(tab)
                        setFilterParams({
                            status: 'IN_PROGRESS',
                        });
                    } else if (tab === 'drafts') {
                        console.log(tab)
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
