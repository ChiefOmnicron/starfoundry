import { Alert, Button, Center, Group, Stack, Title } from '@mantine/core';
import { useDisclosure } from '@mantine/hooks';
import { LoadingAnimation } from '@starfoundry/components/misc/LoadingAnimation';
import { LoadingError } from '@starfoundry/components/misc/LoadingError';
import { useListTags } from '@starfoundry/components/services/tags/list';
import { triggerTag } from '@starfoundry/components/services/tags/trigger';
import { CreateTag } from '@starfoundry/components/tags/Create';
import { TagList } from '@starfoundry/components/tags/List';
import { ModalWrapper } from '@starfoundry/components/wrapper/Modal';
import { useMutation } from '@tanstack/react-query';
import { createFileRoute } from '@tanstack/react-router'
import { useState } from 'react';

export const Route = createFileRoute('/tags/')({
    beforeLoad: async ({ context }) => {
        if (!await context.auth.isAuthenticated()) {
            throw context.auth.login();
        }
    },
    component: RouteComponent,
})

function RouteComponent() {
    const [opened, { open, close }] = useDisclosure(false);

    const [hasError, setHasError] = useState<boolean>(false);
    const [isSuccess, setIsSuccess] = useState<boolean>(false);

    const triggerMutation = useMutation({
        mutationFn: async () => {
            return await triggerTag();
        },
        onError: () => {
            setHasError(true);
            setIsSuccess(false);
        },
        onSuccess: () => {
            setIsSuccess(false);
            setHasError(false);
        },
    });

    let {
        isPending,
        isError,
        data: tags,
    } = useListTags({
        auto: true,
        manual: true,
    });

    if (isPending) {
        return <LoadingAnimation />;
    }
    if (isError) {
        return <LoadingError />;
    }

    const addTag = () => {
        return <ModalWrapper
            opened={ opened }
            close={ close }
            title="Add tag"
        >
            <CreateTag
                onCreate={() => close()}
            />
        </ModalWrapper>
    }

    const actionBar = () => {
        return <Group
            align='center'
            justify='flex-end'
            pb='sm'
        >
            <Button
                variant='filled'
                onClick={() => triggerMutation.mutate()}
                disabled={triggerMutation.isPending}
                loading={triggerMutation.isPending}
            >
                Trigger
            </Button>

            <Button
                variant='filled'
                onClick={ open }
            >
                Create Tag
            </Button>
        </Group>
    }


    const notification = () => {
        if (hasError) {
            return <Alert
                mt="sm"
                variant='light'
                color='red'
                title='Error'
                data-cy="successfulUpdate"
                onClose={ () => setHasError(false) }
                withCloseButton
            >
                Error while triggering tags
            </Alert>;
        }
        if (isSuccess) {
            return <Alert
                mt="sm"
                variant='light'
                color='red'
                title='Success'
                data-cy="successfulUpdate"
                onClose={ () => setIsSuccess(false) }
                withCloseButton
            >
                Tags successful triggered
            </Alert>;
        }
    }

    const content = () => {
        if (isPending) {
            return LoadingAnimation();
        } else if (isError) {
            return LoadingError();
        } else if (tags && tags.length > 0) {
            return <TagList
                tags={tags || []}
            />
        } else {
            return <>
                <Center mt={50} data-cy="noData">
                    <Stack>
                        <Title order={4}>No tags yet</Title>

                        <Button
                            variant='filled'
                            onClick={ open }
                        >
                            Create Tag
                        </Button>
                    </Stack>
                </Center>
            </>
        }
    }

    return <>
        { notification() }

        { addTag() }

        { actionBar() }

        { content() }
    </>
}
