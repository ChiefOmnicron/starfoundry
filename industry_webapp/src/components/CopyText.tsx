import { formatNumber } from "@/utils";
import { Tooltip } from "@mantine/core";
import { useClipboard } from "@mantine/hooks";
import { useEffect, useState } from "react";

export function CopyText({
    value,
    display,

    number = false,
}: CopyTextProps) {
    const [opened, setOpened] = useState<boolean>(false);
    const clipboard = useClipboard();

    useEffect(() => {
        if (opened) {
            setTimeout(() => setOpened(false), 1000);
        }
    }, [opened]);

    const clickEvent = () => {
        setOpened(true);
        clipboard.copy(value);
    }

    const formatValue = () => {
        if (number && value) {
            // using <NumberFormatter> from mantine has a problem with placing
            // the tooltip
            return formatNumber(value as number);
        } else {
            return display || value;
        }
    }

    return <>
        <Tooltip
            opened={opened}
            label="Copied!"
            position="top"
        >
            <span
                onClick={clickEvent}
                style={{
                    cursor: 'pointer'
                }}
            >
                { formatValue() }
            </span>
        </Tooltip>
    </> 
}

export type CopyTextProps = {
    value: string | number | undefined;
    display?: string | number | undefined;

    number?: boolean;
}
