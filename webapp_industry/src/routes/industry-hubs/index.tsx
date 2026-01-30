import { AddIndustryHub } from '@/routes/industry-hubs/-modal/add';
import { Alert, Button, Center, Flex, Modal, Stack, Title } from '@mantine/core';
import { createFileRoute } from '@tanstack/react-router';
import { Filter, type FilterPropEntry, type SelectedFilter } from '@/components/Filter';
import { IndustryHubList } from '@/components/IndustryHubCard';
import { LoadingAnimation } from '@/components/LoadingAnimation';
import { LoadingError } from '@/components/LoadingError';
import { normalizeRigServiceName } from '@/services/structure/utils';
import { useDisclosure } from '@mantine/hooks';
import { useEffect, useState } from 'react';
import { useListIndustryHub } from '@/services/industry-hub/list';
import type { StructureFilter } from '@/services/structure/list';

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

    const [filterParams, setFilterParams] = useState<StructureFilter | undefined>();
    const [filterOptions, setFilterOptions] = useState<FilterPropEntry[]>([]);

    const {
        isPending,
        isError,
        isSuccess,
        data: industryHubs,
    } = useListIndustryHub(filterParams || {});

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
            structure_type_id: filters.find(x => x.filterKey === 'structure_type_id')?.key as number,
            system_id: filters.find(x => x.filterKey === 'system_id')?.key as number,
            service_id: filters.find(x => x.filterKey === 'service_id')?.key as number,
            rig_id: filters.find(x => x.filterKey === 'rig_id')?.key as number,
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
                { actionBar() }

                { filter }
            </>
        } else if (filterParams && isPending) {
            return <>
                { actionBar() }

                { filter }

                { LoadingAnimation() }
            </>
        } else if (filterParams && isSuccess && industryHubs.length === 0) {
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

    return <>
        { notification() }

        { content() }

        <IndustryHubList
            industryHubs={industryHubs || []}
            industryHubCardProps={{
                editLink: true,
            }}
        />
    </>
}
