import { Button, Card, Flex, Grid, Modal, Stack, Text, TextInput } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { useEffect, useState, type ReactElement } from "react";

export function ArchiveResource({
    isArchived,

    resource,
    onConfirm,
}: ArchiveResourceProps): ReactElement {
    const [opened, { open, close }] = useDisclosure(false);
    const [validation, setValidation] = useState('');

    useEffect(() => {
        setValidation('');
    }, [opened]);

    const confirmModal = () => {
        return <Modal
            data-cy="modalConfirmArchive"
            opened={opened}
            onClose={close}
            title="Confirm archive"
            overlayProps={{
                backgroundOpacity: 0.55,
                blur: 3,
            }}
            size='xl'
            centered
            closeOnEscape
            closeOnClickOutside
        >
            <Stack>
                <Text>
                    Are you sure you want to archive '<b>{ resource }</b>'?
                    This can be reversed at any time
                </Text>

                <TextInput
                    data-cy="confirmArchiveText"
                    label="Insert name to confirm"
                    placeholder={resource}
                    value={validation}
                    onChange={(event) => setValidation(event.currentTarget.value)}
                />

                <Button
                    data-cy="confirmArchive"
                    color="orange"
                    onClick={() => {
                        onConfirm();
                        close();
                    }}
                    disabled={validation.toLocaleLowerCase() !== resource.toLocaleLowerCase()}
                >
                    I know what I am doing, archive it
                </Button>
            </Stack>
        </Modal>
    }

    return <>
        { confirmModal() }

        <Card
            data-cy="archiveCard"
            style={{
                borderColor: 'var(--mantine-color-orange-filled)',
            }}
            mt="sm"
            withBorder
        >
            <Grid>
                <Grid.Col span={6}>
                    <Text
                        style={{
                            fontWeight: 'bold',
                        }}
                    >
                        {
                            isArchived
                            ? `Unarchive '${resource}'`
                            : `Archive '${resource}'`
                        }

                        
                    </Text>
                    <Text
                    >
                        {
                            isArchived
                            ? `'${resource}' will be found in searches again`
                            : `'${resource}' will still be available, but will not show up in searches`
                        }
                        
                    </Text>
                </Grid.Col>
                <Grid.Col span={6}>
                    <Flex
                        justify="flex-end"
                    >
                        <Button
                            data-cy="archive"
                            color="orange"
                            onClick={() => {
                                // shortcut when the resource is archived
                                if (isArchived) {
                                    onConfirm();
                                    return;
                                }

                                open();
                            }}
                        >
                            {
                                isArchived
                                ? 'Unarchive'
                                : 'Archive'
                            }
                        </Button>
                    </Flex>
                </Grid.Col>
            </Grid>
        </Card>
    </>
}

export type ArchiveResourceProps = {
    isArchived: boolean;

    resource: string,
    onConfirm: () => void,
}
