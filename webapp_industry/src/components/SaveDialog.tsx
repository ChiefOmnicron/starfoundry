import { useBlocker } from "@tanstack/react-router";
import { Button, Dialog, Group, Text } from "@mantine/core";
import { useState } from "react";

export function SaveDialog({
    show,
    onSave,
    onReset,
}: Props) {
    const [shakeDialog, setShakeDialog] = useState<boolean>(false);

    useBlocker({
        shouldBlockFn: () => {
            if (show) {
                setShakeDialog(true);

                setTimeout(() => {
                    setShakeDialog(false);
                }, 500);
            }

            return show;
        },
        enableBeforeUnload: true,
    });

    return <>
        <Dialog
            opened={ show }
            size="xl"
            position={{
                bottom: 50,
                right: '35%',
            }}
            className={`${shakeDialog ? 'shakeDialog' : ''}`}
        >
            <Group grow>
                <Text>
                    Unsaved changes
                </Text>

                <Group align="flex-end" justify="end">
                    <Button
                        variant="default"
                        onClick={onReset}
                    >
                        Reset
                    </Button>

                    <Button
                        onClick={onSave}
                    >
                        Save
                    </Button>
                </Group>
            </Group>
      </Dialog>
    </>
}

// checks if two array are the same
//
export function compareArray<T>(
    a: T[],
    b: T[],
): boolean {
    const differenceAB = a.filter(value => !b.includes(value));
    const differenceBA = b.filter(value => !a.includes(value));

    return differenceAB.length === 0 && differenceBA.length === 0;
}

export type Props = {
    show:      boolean;
    onSave():  void;
    onReset(): void;
}
