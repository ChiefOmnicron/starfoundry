import { Button, Flex, Grid, Image, NumberInput, Select, Stack, Table, TagsInput, Textarea, TextInput } from "@mantine/core";
import { CategorySelector } from "@/components/CategorySelector";
import { Route as ProductsRoute } from '@/routes/admin/products/index';
import { useForm } from "@tanstack/react-form";
import { useNavigate } from "@tanstack/react-router";
import { useState, type ReactElement } from "react";
import type { Uuid } from "@/services/utils";
import { useListProducts } from "@/services/product/list";

export type ProductForm = {
    name:               string,
    price:              number | undefined,

    image_type:         string,
    image_type_id:      number,

    description?:       string,
    category?:          string,

    tags:               string[],

    content:            string,
    additional_options: AdditionalOption[],
}

export type AdditionalOption = {
    typ:          'PRODUCT',
    reference_id: Uuid,
}

export type ProductFormComponentProps = {
    product: ProductForm,
    onSubmit: (value: ProductForm) => void
}

export function ProductFormComponent({
    product,
    onSubmit,
}: ProductFormComponentProps): ReactElement {
    const navigation = useNavigate();
    const [imageType, setImageType] = useState<string>(product.image_type);
    const [imageTypeId, setImageTypeId] = useState<number>(product.image_type_id);

    const [additionalRowAdd, setAdditionalRowAdd] = useState<string | null>('');
    const [additionalOptions, setAdditionalOptions] = useState<AdditionalOption[]>(product.additional_options);

    const {
        data: products
    } = useListProducts();

    const form = useForm({
        defaultValues: product,
        onSubmit: async ({ value }) => onSubmit(value),
    });

    const renderOptions = [{
        label: 'Render (only for ships)',
        value: 'render',
    }, {
        label: 'Icon',
        value: 'icon',
    }];

    const productOptions = () => {
        if (!products) {
            return [];
        }

        return products
            .map(x => {
                return {
                    value: x.uuid,
                    label: x.name,
                }
            });
    }

    const additionalOptionsRows = (field: any) => {
        return additionalOptions
            .map(x => {
                return <Table.Tr key="x.reference_id">
                    <Table.Td>{ productOptions().find(y => y.value === x.reference_id)?.label }</Table.Td>
                    <Table.Td>
                        <Button
                            variant="default"
                            fullWidth
                            onClick={() => {
                                const index = field.state
                                    .value
                                    .findIndex((y: AdditionalOption) => y.reference_id === x.reference_id);
                                field.removeValue(index);

                                setAdditionalOptions(
                                    additionalOptions
                                        .filter(y => y.reference_id !== x.reference_id));
                            }}
                        >
                            Delete
                        </Button>
                    </Table.Td>
                </Table.Tr>
            })
    }

    return <form
        onSubmit={(e) => {
            e.preventDefault();
            e.stopPropagation();
            form.handleSubmit();
        }}
    >
        <Stack>
            <form.Field
                name="name"
                validators={{
                    onBlur: ({ value }) => {
                        return (value.trimStart().length === 0 ? 'The field is required' : undefined) ||
                            (value.length > 100 ? 'Maximum allowed chars is 100' : undefined)
                    }
                }}
                children={(field) => {
                    return <>
                        <TextInput
                            data-1p-ignore
                            data-cy="name"
                            label="Name"
                            description="Name of the product"
                            placeholder="My cool product"
                            id={field.name}
                            name={field.name}
                            value={field.state.value}
                            error={
                                !field.state.meta.isValid && field.state.meta.errors.join(', ')
                            }
                            onBlur={field.handleBlur}
                            onChange={(e) => field.handleChange(e.target.value)}
                            withAsterisk
                        />
                    </>
                }}
            />

            <form.Field
                name="category"
                children={(field) => {
                    return <>
                        <CategorySelector
                            onChange={(e) => field.handleChange(e)}
                            defaultValue={field.state.value}
                        />
                    </>
                }}
            />

            <form.Field
                name="description"
                validators={{
                    onBlur: ({ value }) => {
                        return (value && value.length > 10000 ? 'Maximum allowed chars is 10000' : undefined)
                    }
                }}
                children={(field) => {
                    return <>
                        <Textarea
                            data-1p-ignore
                            data-cy="description"
                            label="Description"
                            description="Description of the product"
                            placeholder="This product is very cool and you should buy it"
                            id={field.name}
                            name={field.name}
                            value={field.state.value}
                            error={
                                !field.state.meta.isValid && field.state.meta.errors.join(', ')
                            }
                            onBlur={field.handleBlur}
                            onChange={(e) => field.handleChange(e.target.value)}
                            autosize
                            minRows={3}
                        />
                    </>
                }}
            />

            <form.Field
                name="price"
                validators={{
                    onBlur: ({ value }) => {
                        return (value && value >= 0 ? undefined : 'The price needs to be at least 0')
                    }
                }}
                children={(field) => {
                    return <>
                        <NumberInput
                            data-1p-ignore
                            data-cy="price"
                            label="Price"
                            description="Price of the product"
                            placeholder="1,000,000,000 ISK"
                            thousandSeparator=","
                            id={field.name}
                            name={field.name}
                            value={field.state.value}
                            min={0}
                            allowNegative={false}
                            onBlur={field.handleBlur}
                            onChange={(e) => field.handleChange(e as number)}
                            suffix=" ISK"
                            withAsterisk
                            error={
                                !field.state.meta.isValid && field.state.meta.errors.join(', ')
                            }
                        />
                    </>
                }}
            />

            <Grid>
                <Grid.Col span={6}>
                    <form.Field
                        name="image_type_id"
                        children={(field) => {
                            return <>
                                <NumberInput
                                    data-cy="image_type_id"
                                    label="Image Type id"
                                    description="Determines what image should be shown"
                                    id={field.name}
                                    name={field.name}
                                    value={field.state.value}
                                    onBlur={field.handleBlur}
                                    min={0}
                                    allowNegative={false}
                                    onChange={(e) => {
                                        field.handleChange(e as number);
                                        setImageTypeId(e as number);
                                    }}
                                    withAsterisk
                                />
                            </>
                        }}
                    />
                </Grid.Col>

                <Grid.Col span={5}>
                    <form.Field
                        name="image_type"
                        children={(field) => {
                            return <>
                                <Select
                                    data={renderOptions}
                                    label="Image Type"
                                    description="Depending on the selection the type of image loaded from the image server will change"
                                    id={field.name}
                                    name={field.name}
                                    value={field.state.value}
                                    onBlur={field.handleBlur}
                                    onChange={(e) => {
                                        field.handleChange(e as string);
                                        setImageType(e as string);
                                    }}
                                    withAsterisk
                                />
                            </>
                        }}
                    />
                </Grid.Col>

                <Grid.Col span={1}>
                    <Image
                        src={`https://images.evetech.net/types/${imageTypeId}/${imageType}?size=128`}
                        h={128}
                        w={128}
                    />
                </Grid.Col>
            </Grid>

            <form.Field
                name="tags"
                children={(field) => {
                    return <>
                        <TagsInput
                            id={field.name}
                            label="Select or create tags"
                            description="Select or create tags"
                            placeholder="Press enter to create"
                            data={[]}
                            value={field.state.value}
                            onBlur={field.handleBlur}
                            acceptValueOnBlur={false}
                            onChange={(e) => field.handleChange(e)}
                            withAsterisk
                        />
                    </>
                }}
            />

            <form.Field
                name="content"
                validators={{
                    onBlur: ({ value }) => {
                        return (value && value.length > 10000 ? 'Maximum allowed chars is 10000' : undefined)
                    }
                }}
                children={(field) => {
                    return <>
                        <Textarea
                            data-1p-ignore
                            data-cy="content"
                            label="Content"
                            description="List everything the buyer gets"
                            placeholder="This and that"
                            id={field.name}
                            name={field.name}
                            value={field.state.value}
                            error={
                                !field.state.meta.isValid && field.state.meta.errors.join(', ')
                            }
                            onBlur={field.handleBlur}
                            onChange={(e) => field.handleChange(e.target.value)}
                            autosize
                            minRows={3}
                            withAsterisk
                        />
                    </>
                }}
            />

            <form.Field
                name="additional_options"
                children={(field) => {
                    return <>
                        <Table>
                            <Table.Thead>
                                <Table.Tr>
                                    <Table.Th>Reference</Table.Th>
                                    <Table.Th w={100}></Table.Th>
                                </Table.Tr>
                            </Table.Thead>

                            <Table.Tbody>{ additionalOptionsRows(field) }</Table.Tbody>

                            <Table.Tfoot>
                                <Table.Tr>
                                    <Table.Td>
                                        <Select
                                            data={productOptions()}
                                            value={additionalRowAdd ? additionalRowAdd : null}
                                            onChange={setAdditionalRowAdd}
                                            placeholder="Select another product as an optional addition"
                                        />
                                    </Table.Td>
                                    <Table.Td>
                                        <Button
                                            variant="default"
                                            fullWidth
                                            onClick={() => {
                                                field.pushValue({
                                                    typ: 'PRODUCT',
                                                    reference_id: additionalRowAdd as string,
                                                });

                                                setAdditionalOptions([
                                                    ...additionalOptions,
                                                    {
                                                        typ: 'PRODUCT',
                                                        reference_id: additionalRowAdd as string,
                                                    }
                                                ]);
                                                setAdditionalRowAdd(null);
                                            }}
                                        >
                                            Add
                                        </Button>
                                    </Table.Td>
                                </Table.Tr>
                            </Table.Tfoot>
                        </Table>
                    </>
                }}
            />

            <form.Subscribe
                selector={(state) => [state.canSubmit, state.isSubmitting]}
                children={([canSubmit, isSubmitting]) => (
                    <Flex
                        justify="flex-end"
                        gap="sm"
                    >
                        <Button
                            variant="subtle"
                            color="gray"
                            onClick={() => navigation({ to: ProductsRoute.to })}
                        >
                            Back
                        </Button>
                        <Button
                            data-cy="create"
                            type="submit"
                            disabled={!canSubmit || isSubmitting}
                            loading={isSubmitting}
                        >
                            Create
                        </Button>
                    </Flex>
                )}
            />
        </Stack>
    </form>
}
