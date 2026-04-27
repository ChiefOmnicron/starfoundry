import { formatDate, formatNumber, formatNumberUnit } from "@internal/utils";
import { Tooltip } from "@mantine/core";
import { useClipboard } from "@mantine/hooks";
import { useEffect, useState } from "react";

export function CopyText({
    value,
    display,

    suffix = '',

    date = false,
    dateUtc = false,
    disabled = false,
    number = false,
    withUnit = false,
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
            if (withUnit) {
                return `${formatNumber(value as number)} ${suffix} (${formatNumberUnit(value as number)})`;
            } else {
                return `${formatNumber(value as number)} ${suffix}`;
            }
        } else if (date && value) {
            return `${formatDate(value as number)} (local)`;
        } else if (dateUtc && value) {
            return `${formatDate(value as number)} (UTC)`;
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
                    cursor: 'pointer',
                    color: disabled ? 'var(--mantine-color-disabled-color)' : '',
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

    suffix?: string;

    date?: boolean;
    dateUtc?: boolean;
    disabled?: boolean;
    number?: boolean;
    withUnit?: boolean;
}
