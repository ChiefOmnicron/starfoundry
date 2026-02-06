import { Button, Card, Flex, Grid, Modal, Stack, Text, TextInput } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { useEffect, useState, type ReactElement } from "react";

export function DeleteResource({
        resource,
        onConfirm,
    }: DeleteResourceProps,
): ReactElement {
    const [opened, { open, close }] = useDisclosure(false);
    const [validation, setValidation] = useState('');

    useEffect(() => {
        setValidation('');
    }, [opened]);

    return <>
        <Modal
            data-cy="modalConfirmDelete"
            opened={ opened }
            onClose={ close }
            title="Confirm delete"
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
                    Are you sure you want to delete '<b>{ resource }</b>'?
                    This cannot be reversed!
                </Text>

                <TextInput
                    data-cy="confirmDeleteText"
                    label="Insert name to confirm"
                    placeholder={resource}
                    value={validation}
                    onChange={(event) => setValidation(event.currentTarget.value)}
                />

                <Button
                    data-cy="confirmDelete"
                    color="red"
                    onClick={() => {
                        onConfirm();
                        close();
                    }}
                    disabled={validation.toLocaleLowerCase() !== resource.toLocaleLowerCase()}
                >
                    I know what I am doing, delete it
                </Button>
            </Stack>
        </Modal>

        <Card
            data-cy="danger-zone-card"
            style={{
                borderColor: 'var(--mantine-color-red-filled)',
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
                        Delete '{ resource }'
                    </Text>
                    <Text
                    >
                        This can not be reversed
                    </Text>
                </Grid.Col>
                <Grid.Col span={6}>
                    <Flex
                        justify="flex-end"
                    >
                        <Button
                            data-cy="delete"
                            color="red"
                            onClick={ () => open() }
                        >
                            Delete
                        </Button>
                    </Flex>
                </Grid.Col>
            </Grid>
        </Card>
    </>
}

export type DeleteResourceProps = {
    resource: string,
    onConfirm: () => void,
}
