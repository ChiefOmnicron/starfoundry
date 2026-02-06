import { Alert, Avatar, Button, Flex, Grid, List, NumberFormatter, ScrollArea, Stack, Table, Textarea, TextInput, Title } from '@mantine/core'
import { createColumnHelper, flexRender, getCoreRowModel, useReactTable } from '@tanstack/react-table';
import { createFileRoute, useNavigate } from '@tanstack/react-router'
import { DELIVERY_SYSTEMS } from '@/services/deliverySystem';
import { FETCH_ORDER, useFetchOrder } from '@/services/order/fetch';
import { LoadingAnimation } from '@/components/LoadingAnimation';
import { LoadingError } from '@/components/LoadingError';
import { Route as OrderRoute } from '@/routes/orders/index';
import { type OrderProduct } from '@/services/order/list';
import { updateOrderComment, type UpdateOrder } from '@/services/order/update';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { useForm } from '@tanstack/react-form';
import { useState } from 'react';

export const Route = createFileRoute('/orders_/$orderUuid/')({
    component: RouteComponent,
})

function RouteComponent() {
    const navigation = useNavigate({ from: Route.fullPath });
    const { orderUuid } = Route.useParams();

    const [errorUpdate, setErrorUpdate] = useState<boolean>(false);
    const [updateSuccess, setUpdateSuccess] = useState<boolean>(false);

    const {
        isPending,
        isError,
        data: order
    } = useFetchOrder(orderUuid);

    const queryClient = useQueryClient();
    const update = useMutation({
        mutationFn: async (data: UpdateOrder) => {
            return await updateOrderComment(orderUuid, data);
        },
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: [FETCH_ORDER, orderUuid] })
        },
    });

    if (isPending) {
        return LoadingAnimation();
    }

    if (isError) {
        return LoadingError();
    }

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

    const form = useForm({
        defaultValues: {
            comment: order.comment,
        },
        onSubmit: async ({ value }) => await update
            .mutateAsync(value)
            .then(_ => {
                setUpdateSuccess(true)
            })
            .catch(_ => {
                setErrorUpdate(true);
            }),
    });

    const notification = () => {
        if (errorUpdate) {
            return <Alert
                variant='light'
                color='red'
                title='Update error'
                onClose={ () => setErrorUpdate(false) }
                withCloseButton
                closeButtonLabel="Dismiss"
            >
                There was an error while updating the order. Please try again later.
            </Alert>;
        } else if (updateSuccess) {
            return <Alert
                variant='light'
                color='green'
                title='Update success'
                onClose={ () => setUpdateSuccess(false) }
                withCloseButton
                closeButtonLabel="Dismiss"
            >
                The order was successfully updated.
            </Alert>;
        }
    };

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
        let info = DELIVERY_SYSTEMS.find(x => x.value === order.delivery_location);
        if (!info) {
            return 'Unknown delivery system';
        } else {
            return info.label;
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

    return <>
        { notification() }

        <form
            onSubmit={(e) => {
                e.preventDefault();
                e.stopPropagation();
                form.handleSubmit();
            }}
        >
            <Stack style={{ width: '100%' }}>
                <Grid>
                    <Grid.Col span={{ base: 12, sm: 8}}>
                        <Stack>
                            <Title order={2}>Configuration</Title>

                            <TextInput
                                label="Delivery Location"
                                value={deliverySystem()}
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

                            <form.Field
                                name="comment"
                                children={(field) => {
                                    return <>
                                        <Textarea
                                            label="Comment"
                                            id={field.name}
                                            name={field.name}
                                            value={field.state.value}
                                            onBlur={field.handleBlur}
                                            onChange={(e) => field.handleChange(e.target.value)}
                                            autosize
                                            minRows={3}
                                        />
                                    </>
                                }}
                            />
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

                <form.Subscribe
                    selector={(state) => [state.canSubmit, state.isSubmitting]}
                    children={([canSubmit, isSubmitting]) => (
                        <Grid>
                            <Grid.Col span={{ base: 12, sm: 8}}>
                                <Flex
                                    justify="flex-end"
                                    gap="sm"
                                >
                                    <Button
                                        mt="sm"
                                        variant="subtle"
                                        color="gray"
                                        onClick={() => navigation({ to: OrderRoute.to })}
                                    >
                                        Back
                                    </Button>
                                    <Button
                                        data-cy="create"
                                        mt="sm"
                                        type="submit"
                                        disabled={!canSubmit || isSubmitting}
                                        loading={isSubmitting}
                                    >
                                        Save
                                    </Button>
                                </Flex>
                            </Grid.Col>
                        </Grid>
                    )}
                />
            </Stack>
        </form>
    </>
}
