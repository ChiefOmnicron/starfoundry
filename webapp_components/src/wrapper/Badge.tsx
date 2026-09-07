import { Badge, CloseButton, Group, type MantineColor, type MantineSize } from "@mantine/core"
import type { ReactNode } from "react";

export function BadgeWrapper({
    color,
    size,

    onClick = () => {},
    onRemove = () => {},

    withCloseButton = false,

    children,
}: BadgeWrapperProps) {
    return <Badge
        color={color}
        radius="xs"
        size={size}
        onClick={onClick}
        autoContrast
        style={{
            paddingRight: withCloseButton ? 0 : undefined,
        }}
    >
        {
            withCloseButton
            ?   <Group style={{
                    gap: 0,
                }}>
                    {children}

                    <CloseButton
                        variant="transparent"
                        size='xs'
                        onClick={onRemove}
                    />
                </Group>
            :   children
        }
    </Badge>;
}

export type BadgeWrapperProps = {
    color?: MantineColor;
    size?:  MantineSize,

    onClick?: () => void;
    onRemove?: () => void;

    withCloseButton?: boolean;

    children: ReactNode;
}
