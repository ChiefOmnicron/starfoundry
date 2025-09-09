import { Pill } from "@mantine/core"
import type { ReactElement } from "react"

export type OrderStatusType = 'ACCEPTED' | 'IN_PROGRESS' | 'DELIVERED' | 'CANCELED';

export function OrderStatus({
    status
}: {
    status: OrderStatusType,
}): ReactElement {
    return <Pill size="md">
        { statusToHuman(status) }
    </Pill>
}

function statusToHuman(
    status: OrderStatusType
) {
    switch(status) {
        case 'ACCEPTED':
            return 'Accepted';
        case 'IN_PROGRESS':
            return 'In Progress';
        case 'DELIVERED':
            return 'Delivered';
        case 'CANCELED':
            return 'Canceled'
    }
}
