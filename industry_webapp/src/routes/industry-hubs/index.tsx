import { AddIndustryHub } from '@/routes/industry-hubs/-modal/add';
import { Alert, Button, Center, Flex, Modal, Stack, Tabs, Title } from '@mantine/core';
import { createFileRoute } from '@tanstack/react-router';
import { Filter, type FilterPropEntry, type SelectedFilter } from '@/components/Filter';
import { IndustryHubList } from '@/components/IndustryHubCard';
import { LoadingAnimation } from '@/components/LoadingAnimation';
import { LoadingError } from '@/components/LoadingError';
import { normalizeRigServiceName } from '@/services/structure/utils';
import { useDisclosure } from '@mantine/hooks';
import { useEffect, useState } from 'react';
import { useListIndustryHub, type IndustryHubFilter } from '@/services/industry-hub/list';

interface QueryParams {
    deleted?: boolean;
}

export const Route = createFileRoute('/industry-hubs/')({
    beforeLoad: async ({ context }) => {
        if (!(await context.auth.isAuthenticated())) {
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

function RouteComponent() {
    const [opened, { open, close }] = useDisclosure(false);
    const { deleted: deletedResource } = Route.useSearch();

    const [filterParams, setFilterParams] = useState<IndustryHubFilter>({});
    const [filterOptions, setFilterOptions] = useState<FilterPropEntry[]>([]);

    const {
        isPending,
        isError,
        isSuccess,
        data: industryHubs,
    } = useListIndustryHub(filterParams);

    useEffect(() => {
        if (!isSuccess) {
            return;
        }

        if (filterParams) {
            return;
        }

        const uniqueStructures = new Map();
        (industryHubs || [])
            .flatMap(x => x.structures)
            .map(x => {
                return {
                    label: x.item.name,
                    key:   x.item.type_id,
                };
            })
            .map(x => uniqueStructures.set(x.key, x));

        const uniqueSystems = new Map();
        (industryHubs || [])
            .flatMap(x => x.structures)
            .map(x => {
                return {
                    label: x.system.system_name,
                    key:   x.system.system_id,
                };
            })
            .map(x => uniqueSystems.set(x.key, x));

        const uniqueServices = new Map();
        (industryHubs || [])
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
        (industryHubs || [])
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
    }, [industryHubs]);

    const filterChange = (filters: SelectedFilter[]) => {
        setFilterParams({
            name: filters.find(x => x.filterKey === 'name')?.value as string,
        });
    };

    const filter = <Filter
        entries={filterOptions}
        onFilterChange={filterChange}
    />;

    const addIndustryHubModal = () => {
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
            <AddIndustryHub
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
            { addIndustryHubModal() }

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
                    Add Industry Hub
                </Button>
            </Flex>
        </>
    }

    const content = () => {
        if (isPending && !filterParams) {
            return LoadingAnimation();
        } else if (isError) {
            return LoadingError();
        } else if (isSuccess && industryHubs.length > 0) {
            return <>
                { filter }
            </>
        } else if (filterParams && isPending) {
            return <>
                { filter }

                { LoadingAnimation() }
            </>
        } else if (filterParams && isSuccess && industryHubs.length === 0) {
            return <>
                { filter }

                <Center mt={50} data-cy="noData">
                    <Title order={4}>No industry hubs matching</Title>
                </Center>
            </>
        } else {
            return <>
                <Center mt={50} data-cy="noData">
                    <Stack>
                        <Title order={4}>No industry hubs yet</Title>

                        <Button
                            variant='filled'
                            onClick={open}
                        >
                            Add Industry Hub
                        </Button>
                    </Stack>
                </Center>
            </>
        }
    }

    const changeTabs = (tab: string | null) => {
        let shared = tab === 'shared';
        setFilterParams({
            ...filterParams,
            shared,
        });
    }

    return <>
        { notification() }

        { actionBar() }

        <Tabs
            defaultValue="my"
            onChange={changeTabs}
        >
            <Tabs.List>
                <Tabs.Tab value="my">
                    My Industry Hubs
                </Tabs.Tab>

                <Tabs.Tab value="shared">
                    Industry Hubs Shared with me
                </Tabs.Tab>
            </Tabs.List>

            { content() }

            <Tabs.Panel value="my">
                <IndustryHubList
                    industryHubs={industryHubs || []}
                    industryHubCardProps={{
                        editLink: true,
                    }}
                />
            </Tabs.Panel>

            <Tabs.Panel value="shared">
                <IndustryHubList
                    industryHubs={industryHubs || []}
                    industryHubCardProps={{
                        cloneLink: true,
                    }}
                />
            </Tabs.Panel>
        </Tabs>
    </>
}
