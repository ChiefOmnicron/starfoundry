import type { TypeId } from "@internal/services/utils"
import { Image } from "@mantine/core";
import type { ReactElement } from "react";

const BASE_URL = 'https://images.evetech.net';

export function EveIcon({
    id,
    type = 'icon',
    width = 32,
    height = 32,
    category = 'types',
}: EveIconProp): ReactElement {
    const url = `${BASE_URL}/${category}/${id}/${type}?size=1024`;

    return <Image
        src={url}
        style={{
            maxWidth: width,
            maxHeight: height,
            minWidth: width,
            minHeight: height,
        }}
    />
}

export type EveIconProp = {
    id: TypeId,
    type?: 'icon' | 'bp' | 'bpc' | 'render' | 'logo' | 'portrait',
    width?: number,
    height?: number,
    category?: 'alliances' | 'corporations' | 'characters' | 'types',
}
