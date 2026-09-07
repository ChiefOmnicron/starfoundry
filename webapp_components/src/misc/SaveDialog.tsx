import { useBlocker } from "@tanstack/react-router";
import { ActionBar, Button, Text } from "@mantine/core";
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
        disabled: !show,
    });

    return <>
        <ActionBar
            opened={ show }
            className={`${shakeDialog ? 'shakeDialog' : ''}`}
        >
            <Text>
                Unsaved changes
            </Text>

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
        </ActionBar>
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
