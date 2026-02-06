import { Avatar, Button, Grid, List, NumberFormatter, ScrollArea, Select, Stack, Table, Textarea, TextInput, Title } from '@mantine/core'
import { createColumnHelper, flexRender, getCoreRowModel, useReactTable } from '@tanstack/react-table';
import { createFileRoute } from '@tanstack/react-router'
import { DatePickerInput } from '@mantine/dates';
import { LoadingAnimation } from '@/components/LoadingAnimation';
import { LoadingError } from '@/components/LoadingError';
import { type OrderProduct } from '@/services/order/list';
import { updateOrderAdmin } from '@/services/order/update_admin';
import { useEffect, useState } from 'react';
import { useFetchOrderAdmin } from '@/services/order/fetch_admin';
import { useMutation } from '@tanstack/react-query';
import type { OrderStatusType } from '@/components/OrderStatus';

export const Route = createFileRoute('/admin/orders_/$orderUuid/')({
    component: RouteComponent,
})

function RouteComponent() {
    const { orderUuid } = Route.useParams();

    const [orderStatus, setOrderStatus] = useState<OrderStatusType>('ACCEPTED');
    const [expectedDeliveryDate, setExpectedDeliveryDate] = useState<string | null>('');
    const [sfIndustryLink, setSfIndustryLink] = useState<string | undefined>('');

    const {
        isPending,
        isError,
        isSuccess,
        data: order
    } = useFetchOrderAdmin(orderUuid);

    const updateOrder = useMutation({
        mutationFn: () => {
            return updateOrderAdmin(orderUuid, {
                status: orderStatus,
                expected_delivery_date: new Date(expectedDeliveryDate || ''),
                sf_industry_link: sfIndustryLink,
            });
        },
    });

    if (isPending) {
        return LoadingAnimation();
    }

    if (isError) {
        return LoadingError();
    }

    useEffect(() => {
        if (isSuccess) {
            setOrderStatus(order.status);
            setExpectedDeliveryDate(order.expected_delivery_date || '');
            setSfIndustryLink(order.sf_industry_link || '');
        }
    }, [order]);

    const additionalOptionsColumnHelper = createColumnHelper<OrderProduct>();
    const columns = [
        additionalOptionsColumnHelper.display({
            id: 'product',
            cell: info => info.row.original.name,
            header: () => 'Name',
        }),
        additionalOptionsColumnHelper.display({
            id: 'quantity',
            cell: _ => order.quantity,
            header: () => 'Total Price',
        }),
        additionalOptionsColumnHelper.display({
            id: 'price',
            cell: info => {
                const price = info.row.original.price * order.quantity;
                return <NumberFormatter thousandSeparator value={price} />;
            },
            header: () => 'Total Price',
        }),
    ];

    const table = useReactTable<OrderProduct>({
        columns: columns,
        data: order.products,
        autoResetPageIndex: false,
        getCoreRowModel: getCoreRowModel(),
    });

    const additionalOption = () => {
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
    }

    const deliverySystem = () => {
        if (order.delivery_location === 'UALX-3') {
            return 'UALX-3 - Mothership Bellicose (Keepstar)';
        } else if (order.delivery_location === 'C-J6MT') {
            return 'C-J6MT - 1st Taj Mahgoon (Keepstar)';
        } else {
            return 'Unknown';
        }
    }

    const comment = () => {
        if (order.comment) {
            return <Textarea
                label="Delivery Location"
                value={order.comment}
                disabled
                autosize
                minRows={3}
            />
        }
    }

    const contentPrimary = () => {
        return order
            .products
            .filter(x => !x.is_additional)
            .flatMap(x => x.content)
            .map(x => {
                return <List.Item
                    key={x.name}
                    icon={
                        <Avatar
                            src={`https://images.evetech.net/types/${x.type_id}/icon?size=128`}
                            radius={0}
                        />
                    }
                >
                    { `${x.name} x${x.quantity}` }
                </List.Item>
            })
    }

    const contentAddition = () => {
        return order
            .products
            .filter(x => x.is_additional)
            .flatMap(x => x.content)
            .map(x => {
                return <List.Item
                    key={x.name}
                    icon={
                        <Avatar
                            src={`https://images.evetech.net/types/${x.type_id}/icon?size=128`}
                            radius={0}
                        />
                    }
                >
                    { `${x.name} x${x.quantity}` }
                </List.Item>
            })
    }

    const changeStatusView = () => {
        return <Select
            label="Select current status"
            placeholder="Select status"
            data={['ACCEPTED', 'IN_PROGRESS', 'DELIVERED']}
            value={orderStatus}
            onChange={(status: string | null) => setOrderStatus(status as OrderStatusType)}
        />
    }

    const expectedDeliveryDateView = () => {
        return <DatePickerInput
            label="Expected delivery date"
            placeholder="Pick date"
            value={ expectedDeliveryDate }
            onChange={ (x) => setExpectedDeliveryDate(x)}
        />
    }

    const sfIndustryUrlView = () => {
        return <TextInput
            label="StarFoundry Industry Url"
            value={ sfIndustryLink }
            onChange={(x) => setSfIndustryLink(x.currentTarget.value)}
        />
    }

    return <>
        <Stack style={{ width: '100%' }}>
            <Grid>
                <Grid.Col span={{ base: 12, sm: 8}}>
                    <Stack>
                        <Title order={2}>Configuration</Title>

                        <TextInput
                            label="Character"
                            value={ order.character.character_name }
                            disabled
                        />

                        <div>
                            <label style={{
                                fontSize: 'var(--mantine-font-size-sm)',
                                fontWeight: '500',
                                display: 'inline-block'
                            }}
                            >
                                Products
                            </label>
                            { additionalOption() }
                        </div>

                        <TextInput
                            label="Delivery Location"
                            value={ deliverySystem() }
                            disabled
                        />

                        { comment() }

                        { changeStatusView() }

                        { expectedDeliveryDateView() }

                        { sfIndustryUrlView() }
                    </Stack>
                </Grid.Col>

                <Grid.Col span={{ base: 12, sm: 4}}>
                    <Title order={2}>Content</Title>

                    <List>
                        <ScrollArea.Autosize mah={400} type='always'>
                            { contentPrimary() }

                            { contentAddition() }
                        </ScrollArea.Autosize>
                    </List>
                </Grid.Col>
            </Grid>
            
            <Button onClick={() => updateOrder.mutate() }>
                Save
            </Button>
        </Stack>
    </>
}
