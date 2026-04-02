import { faCopy } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { ActionIcon, Tooltip } from "@mantine/core";
import { useClipboard } from "@mantine/hooks";
import { useEffect, useState } from "react";

export function CopyTable({
    value,
}: CopyTableProps) {
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

    return <>
        <Tooltip
            label="Copied!"
            position="top"
            opened={opened}
        >
            <ActionIcon
                color="gray"
                variant="transparent"
                onClick={clickEvent}
            >
                <FontAwesomeIcon icon={faCopy} />
            </ActionIcon>
        </Tooltip>
    </> 
}

export type CopyTableProps = {
    value: string;
}
