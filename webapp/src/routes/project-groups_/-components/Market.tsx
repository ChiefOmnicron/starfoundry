import { StructureSelector } from "@/components/StructureSelector";
import { fetchStructureQuery } from "@/services/structure/fetch";
import type { Uuid } from "@/services/utils";
import { Button, Text } from "@mantine/core";
import { useQueries } from "@tanstack/react-query";
import { createColumnHelper, getCoreRowModel, useReactTable } from "@tanstack/react-table";
import type { ReactElement } from "react";
import { withStructure, withTest123 } from "./TestComponent";

const columnHelper = createColumnHelper<DefaultMarketEntry>();
const columns = [
    // TODO: add unstyled button to the structure
    columnHelper.accessor('name', {
        id: 'name',
        cell: info => info.getValue() /*<UnstyledButton
            component={Link}
            to={`/project-groups/${info.row.original.id}/overview`}
            style={{
                color: 'var(--mantine-color-blue-4)',
                fontSize: 'var(--mantine-font-size-sm)'
            }}
        >
            { info.getValue() }
        </UnstyledButton>*/,
        header: () => 'Name',
    }),
    columnHelper.accessor('id', {
        id: 'delete',
        cell: (_: any) => <Button>
            Delete
        </Button>,
        header: () => 'Projects',
    }),
];

export function ProjectGroupMarket ({
    entries,
}: ProjectGroupMarketProp): ReactElement {
    const structures = useQueries({
        queries: entries.map(x => {
            return fetchStructureQuery(x);
        })
    });

    /*const table = useReactTable<DefaultMarketEntry>({
        columns: columns,
        data: entries,
        autoResetPageIndex: false,
        getCoreRowModel: getCoreRowModel(),
    });*/

    /*

        <Structure
            structureId="f805f279-61ca-4211-8cb6-71bd551d3bc0"
        >

        </Structure>*/

    return <>

        <Struct
            structureId="f805f279-61ca-4211-8cb6-71bd551d3bc0"
        >

        </Struct>
    </>

    /*return <>
        <StructureSelector
            onSelect={(uuid) => console.log('x', uuid)}
            filters={{
                // Standup Market Hub I
                service_id: 35892
            }}
        />
    </>*/
}

function DefaultMarketEntryInternal(
    props: {
        structureId: string,
    },
): ReactElement {
    console.log(props)
    return <>
        <Text>AAA { props.structure }</Text>
    </>
}

function DefaultMarketEntryInternal2(
    props: {
        structure: string,
    },
): ReactElement {
    console.log(props)
    return <>
        <Text>AAA { props.structure }</Text>
    </>
}
const Structure = withStructure(DefaultMarketEntryInternal)

const Struct = withTest123(DefaultMarketEntryInternal2)

export type ProjectGroupMarketProp = {
    entries: Uuid[];
}

export type DefaultMarketEntry = {
    id: Uuid,
    name: string
}
