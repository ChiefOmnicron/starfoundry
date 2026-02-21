import { Badge, Text } from "@mantine/core"
import type { Item } from "@internal/services/item/model"

export function StructureServiceBadge({
    services,

    size = 'xs',
}: StructureServiceBadgeProps) {
    if (services.length === 0) {
        return <Text>The structure has no services installed</Text>
    }

    const uniqueServices: Item[] = [];
    services
        .map(x => {
            if (!uniqueServices.find(y => y.type_id === x.type_id)) {
                uniqueServices.push(x);
            }
        });

    return uniqueServices
        .map(x => {
            let typeId = x.type_id;
            let color = 'red.9';
            let content = '';
            let sortKey = '';

            switch (x.type_id) {
                case 35877:
                    color = 'red.9';
                    content = 'Super Capitals';
                    sortKey = '9'
                    break;
                case 35878:
                    color = 'blue.9';
                    content = 'Manufacturing';
                    sortKey = '7'
                    break;
                case 35881:
                    color = 'orange.9';
                    content = 'Capitals';
                    sortKey = '8'
                    break;
                case 35886:
                    color = 'yellow.9';
                    content = 'Invention';
                    sortKey = '6'
                    break;
                case 35892:
                    color = 'green.9';
                    content = 'Market';
                    sortKey = '10'
                    break;
                case 35899:
                    color = 'violet.9';
                    content = 'Reprocessing';
                    sortKey = '1'
                    break;
                case 45537:
                    color = 'cyan.9';
                    content = 'Composite Reactions';
                    sortKey = '2'
                    break;
                case 45538:
                    color = 'cyan.9';
                    content = 'Hybrid Reactions';
                    sortKey = '4'
                    break;
                case 45539:
                    color = 'cyan.9';
                    content = 'Biochemical Reactions';
                    sortKey = '3'
                    break;
                case 35891:
                case 45550:
                    color = 'yellow.9';
                    content = 'Research';
                    sortKey = '5'
                    break;
                case 35894:
                    color = 'pink.8';
                    content = 'Cloning Bay';
                    sortKey = '11'
                    break;
                default:
                    break;
            }

            return {
                typeId,
                color,
                content,
                sortKey,
            }
        })
        .sort((a, b) => a.sortKey.localeCompare(b.sortKey))
        .map(x => <Badge
                autoContrast
                key={x.content}
                size={size}
                color={x.color}
                radius='xs'
            >
                {x.content}
            </Badge>
        )

}

export type StructureServiceBadgeProps = {
    services: Item[],

    size?: 'xs' | 'sm' | 'md' | 'lg' | 'xl'
}
