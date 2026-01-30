import type { TypeId } from "@/services/utils"
import { Image } from "@mantine/core";
import type { ReactElement } from "react";

const BASE_URL = 'https://images.evetech.net';

export function EveIcon({
    id,
    type = 'icon',
    width = 32,
    height = 32,
}: EveIconProp): ReactElement {
    const url = `${BASE_URL}/types/${id}/${type}?size=1024`;

    return <Image
        src={url}
        style={{
            maxWidth: width,
            maxHeight: height,
        }}
    />
}

export type EveIconProp = {
    id: TypeId,
    type?: 'icon' | 'bp' | 'bpc' | 'render',
    width?: number,
    height?: number,
}
