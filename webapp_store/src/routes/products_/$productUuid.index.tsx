import { Alert, Avatar, Button, Flex, Grid, List, NumberFormatter, NumberInput, ScrollArea, Select, Stack, Table, Text, Textarea, Title, useMatches } from '@mantine/core'
import { createFileRoute, useNavigate } from '@tanstack/react-router'
import { createOrder } from '@/services/order/create';
import { LIST_ORDER } from '@/services/order/list';
import { LoadingAnimation } from '@/components/LoadingAnimation';
import { LoadingError } from '@/components/LoadingError';
import { Route as OrderRoute } from '@/routes/orders/index';
import { Route as StoreRoute } from '@/routes/products/index';
import { useFetchProduct } from '@/services/product/fetch';
import { useForm } from '@tanstack/react-form';
import { useListProducts, type Product } from '@/services/product/list';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { useState } from 'react';
import type { Uuid } from '@/services/utils';
import { DELIVERY_SYSTEMS } from '@/services/deliverySystem';

export type OrderProduct = {
    additionalOptions?: Uuid,
    deliverySystem: string,
    comment: string,
    quantity: number
}

export const Route = createFileRoute('/products_/$productUuid/')({
    beforeLoad: async ({ context }) => {
        if (!await context.auth.isAuthenticated()) {
            throw context.auth.login();
        }
    },
    component: RouteComponent,
})

function RouteComponent() {
    const navigation = useNavigate({ from: Route.fullPath });
    const [additionalOptionPrice, setAdditionalOptionPrice] = useState(0);
    const [deliveryLocationPrice, setDeliveryLocationPrice] = useState(0);
    const [quantity, setQuantity] = useState(1);
    const [selectedAddition, setSelectedAddition] = useState<string | null>(null);
    const [errorCreate, setErrorCreate] = useState<string | undefined>();

    const { productUuid } = Route.useParams();

    const queryClient = useQueryClient();
    const create = useMutation({
        mutationFn: async (data: OrderProduct) => {
            return await createOrder(productUuid, data);
        },
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: [LIST_ORDER] })
        },
    });

    const {
        isPending,
        isError,
        data: product
    } = useFetchProduct(productUuid);

    const {
        data: products,
    } = useListProducts({});

    if (isPending) {
        return LoadingAnimation();
    }

    if (isError) {
        return LoadingError();
    }

    const form = useForm({
        defaultValues: {
            additionalOptions: '00000000-0000-0000-0000-000000000000',
            deliverySystem: 'UALX-3',
            comment: '',
            quantity: 1,
        },
        onSubmit: async ({ value }) => await create
            .mutateAsync(value)
            .then(x => {
                navigation({
                    to: OrderRoute.to,
                    params: {
                        productUuid: x.id,
                    },
                    search: {
                        created: true,
                    },
                });
            })
            .catch(error => {
                setErrorCreate(error);
            }),
    });

    const optionData = () => {
        let data: { value: string, label: string }[] = [{
            value: '00000000-0000-0000-0000-000000000000',
            label: 'No additions',
        }];
        for (const option of product.additional_options) {
            let entry = products.find(x => x.id === option.reference_id);

            if (!entry) {
                continue;
            }

            data.push({
                value: entry.id,
                label: entry.name,
            });
        }
        return data;
    }

    const deliveryLocations = () => {
        return product
            .delivery_location
            .map(x => {
                let info = DELIVERY_SYSTEMS.find(y => y.structureId === x);
                if (!info) {
                    return {
                        value: 'UNKNOWN',
                        label: 'Unknown delivery system'
                    }
                }
                return info;
            })
    }

    const tableWidth = useMatches({
        base: '100%',
        sm: '30%'
    });

    const contentPrimary = () => {
        return product
            .content
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
        if (!selectedAddition) {
            if (additionalOptionPrice !== 0) {
                setAdditionalOptionPrice(0);
            }
            return [];
        }

        let selectedProduct: Product | undefined = products.find(x => x.id === selectedAddition);
        if (!selectedProduct) {
            if (additionalOptionPrice !== 0) {
                setAdditionalOptionPrice(0);
            }
            return [];
        }

        if (additionalOptionPrice !== selectedProduct.price) {
            setAdditionalOptionPrice(selectedProduct.price);
        }

        return selectedProduct
            .content
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

    const notification = () => {
        if (errorCreate) {
            return <Alert
                variant='light'
                color='red'
                title='Order error'
                onClose={ () => setErrorCreate(undefined) }
            >
                There was an error while ordering. Please try again later.
            </Alert>;
        }
    };

    const message = () => {
        if (product.message) {
            return <Alert
                variant='light'
                color='orange'
                title='Important Message'
            >
                <div dangerouslySetInnerHTML={{ __html: product.message.replace(/\n/g, '<br>') }} />
            </Alert>;
        }
    };

    const additionalOptions = () => {
        if (product.additional_options.length === 0) {
            return <></>
        } else {
            return <form.Field
                name="additionalOptions"
                children={(field) => {
                    return <>
                        <Select
                            data={optionData()}
                            label="Additional options"
                            description="Add some spice to the order"
                            id={field.name}
                            name={field.name}
                            value={field.state.value}
                            onBlur={field.handleBlur}
                            onChange={(e) => {
                                field.handleChange(e as string);
                                setSelectedAddition(e);
                            }}
                            withAsterisk
                            allowDeselect={false}
                        />
                    </>
                }}
            />
        }
    }

    return <>
        { notification() }

        { message() }

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

                            { additionalOptions() }

                            <form.Field
                                name="deliverySystem"
                                children={(field) => {
                                    return <>
                                        <Select
                                            data={deliveryLocations()}
                                            label="Delivery location"
                                            description="Select a system the order should be delivered to"
                                            id={field.name}
                                            name={field.name}
                                            value={field.state.value}
                                            onBlur={field.handleBlur}
                                            onChange={(e) => {
                                                field.handleChange(e as string);
                                                setDeliveryLocationPrice(0);
                                            }}
                                            withAsterisk
                                            allowDeselect={false}
                                        />
                                    </>
                                }}
                            />

                            <form.Field
                                name="quantity"
                                children={(field) => {
                                    return <>
                                        <NumberInput
                                            label="Quantity"
                                            description="Quantity you want to buy"
                                            placeholder="1"
                                            id={field.name}
                                            name={field.name}
                                            value={field.state.value}
                                            error={
                                                !field.state.meta.isValid && field.state.meta.errors.join(', ')
                                            }
                                            onBlur={field.handleBlur}
                                            onChange={(e) => {
                                                field.handleChange(e as number);
                                                setQuantity(e as number);
                                            }}
                                            withAsterisk
                                            min={1}
                                        />
                                    </>
                                }}
                            />

                            <form.Field
                                name="comment"
                                children={(field) => {
                                    return <>
                                        <Textarea
                                            label="Comment"
                                            description="Any comment you want to add"
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

                <Title order={2}>Cost Breakdown</Title>

                <Table
                    style={{
                        width: tableWidth
                    }}
                >
                    <Table.Thead>
                        <Table.Tr>
                            <Table.Th>Entry</Table.Th>
                            <Table.Th>Qty.</Table.Th>
                            <Table.Th>Price</Table.Th>
                        </Table.Tr>
                    </Table.Thead>

                    <Table.Tbody>
                        <Table.Tr>
                            <Table.Td>Base price</Table.Td>
                            <Table.Td>
                                <NumberFormatter
                                    value={ quantity }
                                    thousandSeparator
                                />
                            </Table.Td>
                            <Table.Td>
                                <NumberFormatter
                                    value={ product.price * quantity }
                                    thousandSeparator
                                />
                            </Table.Td>
                        </Table.Tr>
                        <Table.Tr>
                            <Table.Td>Additional option</Table.Td>
                            <Table.Td>
                                {
                                    additionalOptionPrice > 0
                                    ? <NumberFormatter
                                        value={ quantity }
                                        thousandSeparator
                                    />
                                    : <></>
                                }
                            </Table.Td>
                            <Table.Td>
                                <NumberFormatter
                                    value={ additionalOptionPrice * quantity }
                                    thousandSeparator
                                />
                            </Table.Td>
                        </Table.Tr>
                        <Table.Tr>
                            <Table.Td colSpan={2}>Delivery location</Table.Td>
                            <Table.Td>
                                <NumberFormatter
                                    value={ deliveryLocationPrice }
                                    thousandSeparator
                                />
                            </Table.Td>
                        </Table.Tr>
                    </Table.Tbody>

                    <Table.Tfoot>
                        <Table.Tr>
                            <Table.Td colSpan={2}>
                                <b>
                                    Total
                                </b>
                            </Table.Td>
                            <Table.Td>
                                <b>
                                    <NumberFormatter
                                        value={ (product.price + additionalOptionPrice) * quantity }
                                        thousandSeparator
                                    />
                                </b>
                            </Table.Td>
                        </Table.Tr>
                    </Table.Tfoot>
                </Table>

                <Title order={2}>Delivery time</Title>
                <Title order={4}>{ product.delivery_time }</Title>
                <Text>This is only an estimated, please give us some time at the start to improve the processes in the background</Text>

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
                                        onClick={() => navigation({ to: StoreRoute.to })}
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
                                        Order
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
