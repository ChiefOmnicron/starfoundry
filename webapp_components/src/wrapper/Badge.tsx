import { Badge, type MantineColor, type MantineSize } from "@mantine/core"
import type { ReactNode } from "react";

export function BadgeWrapper({
    color,

    onClick = () => {},
    size,

    children,
}: BadgeWrapperProps) {
    return <Badge
        color={color}
        radius="xs"
        size={size}
        onClick={onClick}
        autoContrast
    >
        {children}
    </Badge>;
}

export type BadgeWrapperProps = {
    color?: MantineColor;
    size?:  MantineSize,

    onClick?: () => void;

    children: ReactNode;
}
