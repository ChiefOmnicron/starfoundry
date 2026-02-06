import { Alert, Button, Center, Flex, Stack, Title } from '@mantine/core';
import { createFileRoute } from '@tanstack/react-router'
import { IndustryHubList } from '@/components/IndustryHubCard';
import { LoadingAnimation } from '@/components/LoadingAnimation';
import { LoadingError } from '@/components/LoadingError';
import { LIST_PROJECT_GROUP_INDUSTRY_HUBS, useListProjectGroupIndustryHubs } from '@/services/project-group/listIndustryHubs';
import { useDisclosure } from '@mantine/hooks';
import { IndustryHubSelectorModal } from '@/components/selectors/IndustryHubSelector';
import { useListIndustryHub, type IndustryHub } from '@/services/industry-hub/list';
import { useEffect, useState } from 'react';
import { SaveDialog } from '@/components/SaveDialog';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { updateIndustryHubs } from '@/services/project-group/updateIndustryHubs';

export const Route = createFileRoute(
    '/project-groups_/$projectGroupId/industry-hubs',
)({
    component: RouteComponent,
});

function RouteComponent() {
    const queryClient = useQueryClient();
    const { projectGroupId } = Route.useParams();
    const [opened, { open, close }] = useDisclosure(false);

    const [selectedIndustryHubs, setSelectedIndustryHubs] = useState<IndustryHub[]>([]);

    const [successfulUpdate, setSuccessfulUpdate] = useState<boolean>();
    const [errorUpdate, setErrorUpdate] = useState<string | undefined>();

    const {
        isPending: isPendingSelectedIndustryHubs,
        isError: isErrorSelectedIndustryHubs,
        data: selectedIndustryHubsApi
    } = useListProjectGroupIndustryHubs(projectGroupId);

    const {
        isPending: isPendingIndustryHubs,
        isError: isErrorIndustryHubs,
        data: industryHubs
    } = useListIndustryHub({}, !!selectedIndustryHubsApi);

    const mutationUpdateIndustryHubs = useMutation({
        mutationFn: () => updateIndustryHubs(
            projectGroupId,
            selectedIndustryHubs.map(x => x.id)
        ),
        onSuccess: () => {
            setErrorUpdate(undefined);
            setSuccessfulUpdate(true);
            queryClient.invalidateQueries({ queryKey: [LIST_PROJECT_GROUP_INDUSTRY_HUBS] });
        },
        onError: (error) => {
            setErrorUpdate(error as any);
            setSuccessfulUpdate(false);
        }
    });

    useEffect(() => {
        if (selectedIndustryHubsApi) {
            setSelectedIndustryHubs(selectedIndustryHubsApi);
        }
    }, [selectedIndustryHubsApi]);

    if (isPendingSelectedIndustryHubs || isPendingIndustryHubs) {
        return LoadingAnimation();
    }

    if (isErrorSelectedIndustryHubs || isErrorIndustryHubs) {
        return LoadingError();
    }

    const onIndustryHubSelect = (industryHubs: IndustryHub[]) => {
        setSelectedIndustryHubs(industryHubs);
        close();
    }

    const reset = () => {
        setSelectedIndustryHubs(selectedIndustryHubsApi);
    }

    const save = async () => {
        await mutationUpdateIndustryHubs.mutateAsync();
    }

    const notification = () => {
        if (successfulUpdate) {
            return <Alert
                mt="sm"
                variant='light'
                color='green'
                title='Update successful'
                data-cy="updateSuccessful"
            >
                Updating the industry hubs was successful
            </Alert>;
        } else if (errorUpdate) {
            return <Alert
                mt="sm"
                variant='light'
                color='red'
                title='Update error'
                data-cy="errorUpdate"
                onClose={ () => setErrorUpdate(undefined) }
                withCloseButton
            >
                There was an error while updating. Please try again later.
            </Alert>;
        } else {
            return <></>
        }
    }

    const content = () => {
        if (!industryHubs || (industryHubs && industryHubs.length === 0)) {
            return <Center mt={50} data-cy="noData">
                <Stack>
                    <Title order={4}>No industry hubs defined</Title>
                </Stack>
            </Center>
        } else if (industryHubs.length > 0) {
            return <>
                <IndustryHubSelectorModal
                    opened={opened}
                    onClose={close}
                    onSelect={onIndustryHubSelect}

                    industryHubs={industryHubs}
                    selected={selectedIndustryHubs}
                />

                <Stack>
                    <Flex
                        justify='end'
                    >
                        <Button
                            onClick={open}
                        >
                            Edit industry hubs
                        </Button>
                    </Flex>

                    <IndustryHubList
                        industryHubs={selectedIndustryHubs}
                    />
                </Stack>
            </>
        }
    }

    return <>
        { notification() }

        { content() }

        <SaveDialog
            onReset={() => {
                reset();
            }}
            onSave={() => {
                save();
            }}
            show={ selectedIndustryHubsApi !== selectedIndustryHubs }
        />
    </>
}

