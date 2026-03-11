import { formatTime } from "@internal/utils";
import { Badge } from "@mantine/core";
import { useEffect, useState } from "react";

export function Countdown({
    endDate
}: CountdownProps) {
    const [remaining, setRemaining] = useState(0);

    const updateTime = () => {
        const start: any = new Date();
        const end: any = new Date(endDate);
        const remaining = Math.floor((end - start) / 1000);

        setRemaining(remaining > 0 ? remaining : 0);
    }

    useEffect(() => {
        updateTime();

        setInterval(() => {
            updateTime();
        }, 1000);
    }, []);

    if (remaining > 0) {
        return <>
            { formatTime(remaining) }
        </>
    } else {
        return <>
            <Badge
                autoContrast
                color="green.9"
                radius='xs'
            >
                Done
            </Badge>
        </>
    }
}

export type CountdownProps = {
    endDate: string,
}
