import { createColumnHelper, flexRender, getCoreRowModel, useReactTable } from '@tanstack/react-table';
import { createFileRoute } from '@tanstack/react-router'
import { Table } from '@mantine/core';
import { useListProjectMarket, type ProjectMarketEntry } from '@starfoundry/components/services/projects/listMarket';
import { EveIcon } from '@starfoundry/components/misc/EveIcon';
import { CopyText } from '@starfoundry/components/misc/CopyText';
import { LoadingAnimation } from '@starfoundry/components/misc/LoadingAnimation';
import { LoadingError } from '@starfoundry/components/misc/LoadingError';

export const Route = createFileRoute('/projects_/$projectId/market')({
    component: RouteComponent,
});

const source = (source: number): string => {
    switch (source) {
        case 1049588174021:
            return 'C-J'
        case 1046664001931:
            return 'UALX'
        case 60003760:
            return 'Jita'
        case 60008494:
            return 'Amarr'
        default:
            return 'Unknown ' + source
    }
}

const columnHelper = createColumnHelper<ProjectMarketEntry>();
const columns = [
    columnHelper.display({
        id: 'icon',
        cell: props => <EveIcon
            id={props.row.original.item.type_id}
        />,
        size: 1,
        maxSize: 1,
    }),
    columnHelper.display({
        id: 'name',
        cell: props => <CopyText
            value={props.row.original.item.name}
        />,
        header: () => 'Name',
        size: 20,
    }),
    columnHelper.display({
        id: 'quantity',
        cell: props => <CopyText
            value={props.row.original.quantity}
            number
        />,
        header: () => 'Quantity',
        size: 10,
    }),
    columnHelper.display({
        id: 'cost',
        cell: props => <CopyText
            value={props.row.original.cost}
            number
        />,
        header: () => 'Cost',
        size: 10,
    }),
    columnHelper.display({
        id: 'cost_multi',
        cell: props => <>
            <CopyText
                value={(props.row.original.cost_multi.price * props.row.original.cost_multi.quantity) + haulingCost(props.row.original.cost_multi.source, props.row.original.cost_multi.quantity, props.row.original.item.volume)}
                number
            />
            <br />
            {source(props.row.original.cost_multi.source)}
        </>,
        header: () => 'Cost Multi',
        size: 10,
    }),
    columnHelper.display({
        id: 'cost_smart',
        cell: props => {
            const a = props
                .row
                .original
                .cost_smart
                .map(x => {
                    return <>
                        <CopyText
                            value={x.price * x.quantity}
                            number
                        />
                        <br />
                        {source(x.source)}
                        <br />
                    </>
                });

            return <>
                {a}
                <CopyText number value={props.row.original.cost_smart.map(x => x.price * x.quantity).reduce((prev, curr) => prev += curr, 0)} />
                <br />
                Total
            </>
        },
        header: () => 'Cost Smart',
        size: 10,
    }),
    columnHelper.display({
        id: 'cost_smart2',
        cell: _ => <></> /*{
            const a = props
                .row
                .original
                .cost_smart2
                .map(x => {
                    return <>
                        <CopyText
                            value={x.price * x.quantity}
                            number
                        />
                        <br />
                        {source(x.source)}
                        <br />
                    </>
                });

            return <>
                {a}
                <CopyText number value={props.row.original.cost_smart2.map(x => x.price * x.quantity).reduce((prev, curr) => prev += curr, 0)} />
                <br />
                Total
            </>
        },*/,
        header: () => 'Cost Smart 2',
        size: 10,
    }),
];

function RouteComponent() {
    const { projectId } = Route.useParams();

    const {
        isError,
        isPending,
        data: projectMarket,
    } = useListProjectMarket(projectId);

    if (isPending) {
        return LoadingAnimation();
    }

    if (isError) {
        return LoadingError();
    }

    const table = useReactTable<ProjectMarketEntry>({
        columns: columns,
        data: projectMarket.flatMap(x => x.entries),
        autoResetPageIndex: false,
        getCoreRowModel: getCoreRowModel(),
    });

    const findVolume = (type_id: number): number => {
        const item = projectMarket
            .flatMap(x => x.entries)
            .find(x => x.item.type_id === type_id);

        if (item) {
            return item.item.volume;
        } else {
            return 0;
        }
    }

    const total_multi = projectMarket
        .flatMap(x => x.entries)
        .flatMap(x => x.cost_multi)
        .map(x =>
            (x.quantity * x.price) +
            haulingCost(x.source, x.quantity, findVolume(x.type_id))
        )
        .reduce((prev, curr) => prev += curr, 0);
    const total_smart = projectMarket
        .flatMap(x => x.entries)
        .flatMap(x => x.cost_smart)
        .map(x => x.quantity * x.price)
        .reduce((prev, curr) => prev += curr, 0);
    const total_smart2 = 0/*projectMarket
        .flatMap(x => x.entries)
        .flatMap(x => x.cost_smart2)
        .map(x => x.quantity * x.price)
        .reduce((prev, curr) => prev += curr, 0);*/

    return <>
        <>
            <CopyText value={total_multi} number />
            <br />
            <CopyText value={total_smart} number />
            <br />
            <CopyText value={total_smart2} number />
            <br />
            <CopyText value={total_multi - total_smart} number />
            <br />
            <CopyText value={total_multi - total_smart2} number />
        </>

        <Table striped data-cy="data">
            <Table.Thead>
            {table.getHeaderGroups().map(headerGroup => (
                <Table.Tr key={headerGroup.id}>
                    {headerGroup.headers.map(header => (
                        <Table.Th
                            key={header.id}
                            style={{
                                width: `${header.getSize()}%`
                            }}
                        >
                            {flexRender(
                                header.column.columnDef.header,
                                header.getContext()
                            )}
                        </Table.Th>
                    ))}
                </Table.Tr>
            ))}
            </Table.Thead>
            <Table.Tbody>
                {table.getRowModel().rows.map(row => (
                    <Table.Tr key={row.id}>
                        {
                            row.getVisibleCells().map(cell => (
                                <Table.Td key={cell.id}>
                                    {
                                        flexRender(
                                            cell.column.columnDef.cell,
                                            cell.getContext()
                                        )
                                    }
                                </Table.Td>
                            ))
                        }
                    </Table.Tr>
                ))}
            </Table.Tbody>
        </Table>
    </>
}

const haulingCost = (
    structure_id: number,
    quantity: number,
    volume: number,
) => {
    switch (structure_id) {
        case 1046664001931:
            return (quantity * volume) * (19_816_099 / 370_000);
        // C-J
        case 1049588174021:
            return (quantity * volume) * (113_886_795 / 370_000);
        // Jita
        case 60003760:
            return (quantity * volume) * 475
        // Amarr
        case 60008494:
            return (quantity * volume) * (173_566_003 / 370_000)
        default:
            return 0;
    }
}
