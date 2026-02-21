import { Button, Flex, Modal, Stack, Textarea, UnstyledButton } from "@mantine/core";
import type { Item } from "@starfoundry/components/services/item/model";
import { parseItem } from "@starfoundry/components/services/item/parse";
import { useMutation } from "@tanstack/react-query";
import { useState } from "react";

export function StructureScanModal({
    opened,
    onClose,

    onParsed,
}: StructureSelectorModalProp) {
    const [value, setValue] = useState<string>('');
    const [isLoading, setIsLoading] = useState<boolean>(false);

    const parseItemsMutation = useMutation({
        mutationFn: async (value: string) => {
            setIsLoading(true);
            return await parseItem(value);
        },
        onSuccess: (items: Item[]) => {
            setIsLoading(false);
            onParsed(items);
            onClose();
        },
        onError: () => {
            setIsLoading(false);
            // TODO: error
        }
    })

    return <Modal
        opened={opened}
        onClose={onClose}
        title="Structure Scan"
        overlayProps={{
            backgroundOpacity: 0.55,
            blur: 3,
        }}
        size="70%"
        centered
        closeOnEscape
        closeOnClickOutside
    >
        <Stack>
            <Textarea
                name="Scan"
                description='Insert a structure scan, they can be made via Ship Scanner'
                placeholder="TODO: example"
                onChange={(event) => setValue(event.currentTarget.value)}
                rows={10}
            ></Textarea>

            <Flex
                justify='flex-end'
                gap='xs'
            >
                <UnstyledButton
                    onClick={onClose}
                    disabled={isLoading}
                >
                    Close
                </UnstyledButton>
                <Button
                    loading={isLoading}
                    disabled={isLoading}
                    onClick={() => parseItemsMutation.mutateAsync(value)}
                >
                    Parse and Set
                </Button>
            </Flex>
        </Stack>
    </Modal>
}

export type StructureSelectorModalProp = {
    // modal control
    opened: boolean;
    onParsed: (entry: Item[]) => void;
    onClose: () => void;
}
