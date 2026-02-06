import { LoadingAnimation } from '@/components/LoadingAnimation';
import { LoadingError } from '@/components/LoadingError';
import { Alert, Button, Center, NumberFormatter, Stack, Table, Title, UnstyledButton } from '@mantine/core';
import { createFileRoute, Link } from '@tanstack/react-router';
import { createColumnHelper, flexRender, getCoreRowModel, useReactTable } from '@tanstack/react-table';
import { LIST_ORDER, useListOrders, type Order } from '@/services/order/list';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import type { Uuid } from '@/services/utils';
import { deleteOrder } from '@/services/order/delete';
import { OrderStatus } from '@/components/OrderStatus';

type QueryParams = {
    created?: boolean;
}

export const Route = createFileRoute('/orders/')({
    component: OrderListComponent,
    validateSearch: (params: {
        created: boolean,
    }): QueryParams => {
        return {
            created: (params.created) || undefined
        };
    }
});

function OrderListComponent() {
    const { created } = Route.useSearch();

    const queryClient = useQueryClient();
    const deleteMutation: any = useMutation({
        mutationFn: async (orderUuid: Uuid) => {
            return await deleteOrder(orderUuid);
        },
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: [LIST_ORDER] })
        },
    });

    const columnHelper = createColumnHelper<Order>();
    const columns = [
        columnHelper.display({
            id: 'product',
            cell: info => <UnstyledButton
                component={Link}
                to={
                    `/orders/${info.row.original.id}`
                }
                style={{
                    color: 'var(--mantine-color-blue-4)',
                    fontSize: 'var(--mantine-font-size-sm)'
                }}
            >
                { () => {
                    if (info.row.original.products.length === 1) {
                        return info.row.original.products[0].name;
                    } else {
                        return info
                            .row
                            .original
                            .products
                            .map(x => {
                                return <>{ x.name } <br /></>
                            });
                    }
                    }
                }
            </UnstyledButton>,
            header: () => 'Product',
        }),
        columnHelper.display({
            id: 'Count',
            cell: info => {
                let quantity = info.row.original.quantity;
                return <NumberFormatter thousandSeparator value={quantity} />;
            },
            header: () => 'Quantity',
        }),
        columnHelper.display({
            id: 'Price',
            cell: info => {
                const price = info
                    .row
                    .original
                    .products
                    .map(x => x.price)
                    .reduce((prev, curr) => prev + curr, 0);
                const withQuantity = info.row.original.quantity * price;
                return <NumberFormatter thousandSeparator value={withQuantity} />;
            },
            header: () => 'Total Price',
        }),
        columnHelper.accessor('status', {
            id: 'status',
            cell: info => <OrderStatus status={info.getValue()} />,
            header: () => 'Status',
        }),
        columnHelper.accessor('delivery_location', {
            id: 'delivery_location',
            cell: info => info.getValue(),
            header: () => 'Delivery Location',
        }),
        columnHelper.accessor('ordered_at', {
            id: 'ordered_at',
            cell: info => new Date(info.getValue()).toLocaleDateString(),
            header: () => 'Order Date',
        }),
        columnHelper.accessor('expected_delivery_date', {
            id: 'expected_delivery_date',
            cell: info => {
                if (!info.getValue()) {
                    return 'TBD';
                } else {
                    return new Date(info.getValue() || '').toLocaleDateString()
                }
            },
            header: () => 'Expected delivery',
        }),
        columnHelper.display({
            id: 'cancel',
            cell: info => info.row.original.status === 'ACCEPTED' ? <Button
                    onClick={() => deleteMutation.mutate(info.row.original.id)}
                >Cancel</Button>
            : <></>,
            header: () => '',
        }),
    ];

    const {
        isPending,
        isError,
        isFetching,
        data: orders,
    } = useListOrders();

    const table = useReactTable<Order>({
        columns: columns,
        data: orders,
        autoResetPageIndex: false,
        getCoreRowModel: getCoreRowModel(),
    });

    const notification = () => {
        if (created) {
            return <Alert
                mt="sm"
                variant='light'
                color='green'
                title='Order successful'
                data-cy="deleteSuccessful"
            >
                The order was successfully created and will be processed <br />
                You will be notified once the order is done and the contract is up.
            </Alert>;
        }
    }

    const dataTable = () => {
        return <>
            <Table.ScrollContainer minWidth={500}>
                <Table striped data-cy="data">
                    <Table.Thead>
                    {table.getHeaderGroups().map(headerGroup => (
                        <Table.Tr key={headerGroup.id}>
                            {headerGroup.headers.map(header => (
                                <Table.Th key={header.id}>
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
                                    {row.getVisibleCells().map(cell => (
                                        <Table.Td key={cell.id}>
                                            {flexRender(cell.column.columnDef.cell, cell.getContext())}
                                        </Table.Td>
                                    ))}
                                </Table.Tr>
                            ))}
                        </Table.Tbody>
                </Table>
            </Table.ScrollContainer>
        </>
    };

    const content = () => {
        if (isPending || isFetching) {
            return LoadingAnimation();
        } else if (isError) {
            return LoadingError();
        } else if (orders.length > 0) {
            return dataTable();
        } else {
            return <>
                <Center mt={50} data-cy="noData">
                    <Stack>
                        <Title order={4}>No orders yet</Title>
                    </Stack>
                </Center>
            </>
        }
    }

    return <>
        { notification() }

        { content() }
    </>
}
