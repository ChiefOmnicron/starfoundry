import { Modal } from "@mantine/core"
import { useMediaQuery } from "@mantine/hooks";
import type { ReactNode } from "react";

export function ModalWrapper({
    title,

    children,

    opened,
    close,

    size = '70%',
}: ModalWrapperProps) {
    const isMobile = useMediaQuery('(max-width: 50em)');

    return <Modal
        opened={opened}
        onClose={close}
        title={title}
        overlayProps={{
            backgroundOpacity: 0.55,
            blur: 3,
        }}
        size={size}
        fullScreen={isMobile}
        centered
        closeOnEscape
        closeOnClickOutside
    >
        {children}
    </Modal>
}

export type ModalWrapperProps = {
    title: string;

    children: ReactNode;

    opened: boolean;
    close: () => void;

    size?: string;
}
