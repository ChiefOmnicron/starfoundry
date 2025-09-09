import { Button, Modal, Stack, Text, TextInput } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { useState, type ReactElement } from "react";

export function DeleteResourceButton({
        resource,
        onConfirm,
    }: FilterProp,
): ReactElement {
    const [opened, { open, close }] = useDisclosure(false);
    const [validation, setValidation] = useState('');

    return <>
        <Modal
            data-cy="modalConfirmDelete"
            opened={opened}
            onClose={close}
            title="Confirm delete"
                overlayProps={{
                backgroundOpacity: 0.55,
                blur: 3,
            }}
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

        <Button
            data-cy="delete"
            color="red"
            onClick={ () => open() }
        >
            Delete
        </Button>
    </>
}

export type FilterProp = {
    resource: string,
    onConfirm: () => void,
}
