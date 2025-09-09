import { Button, Card, Flex, Image, Pill, SimpleGrid, Stack, Text, Title } from '@mantine/core';
import { createFileRoute, useNavigate } from '@tanstack/react-router';
import { formatNumber, type Uuid } from '@/services/utils';
import { LoadingAnimation } from '@/components/LoadingAnimation';
import { LoadingError } from '@/components/LoadingError';
//import { useState, type ReactElement } from 'react';
import { type ReactElement } from 'react';
import { useListCategory } from '@/services/product/category_list';
import { useListProducts, type Product } from '@/services/product/list';
//import { Filter, type FilterPropEntry, type SelectedFilter } from '@/components/Filter';
import { Route as ProductOrderRoute } from '@/routes/products_/$productUuid.index';

export const Route = createFileRoute('/products/')({
    beforeLoad: async ({ context }) => {
        if (!await context.auth.isAuthenticated()) {
            throw context.auth.login();
        }
    },
    component: StoreComponent,
});

export function StoreComponent() {
    const navigation = useNavigate();

    //const [_, setFilterParams] = useState<Filter>({});
    //const filterChange = (filters: SelectedFilter[]) => {
    //    setFilterParams({
    //        name: filters.find(x => x.key === 'name')?.value as string,
    //    });
    //};

    const {
        isPending: isPendingCategory,
        isError: isErrorCategory,
        data: categories
    } = useListCategory();

    const {
        isPending: isPendingProducts,
        isError: isErrorProducts,
        data: products,
    } = useListProducts();

    /*console.log(products.flatMap(x => x.tags))
    const filters: FilterPropEntry[] = [{
        label: 'Name',
        key: 'name',
        type: 'STRING',
    }, {
        label: 'Tag',
        key: 'tag',
        type: 'MULTISELECT',
        options: Array.from(
                new Set(products.flatMap(x => x.tags))
            ) as any
            .map(x => {
                console.log(x)
                return {
                    label: x,
                    key: x,
                }
            })
    }];*/

    if (isPendingCategory || isPendingProducts) {
        return LoadingAnimation();
    }
    if (isErrorCategory || isErrorProducts) {
        return LoadingError();
    }

    const navigateProduct = (uuid: Uuid) => {
        navigation({
            to: ProductOrderRoute.to,
            params: {
                productUuid: uuid,
            }
        });
    }

    const grouped =  categories
        .map(x => CategoryView(x, products, navigateProduct));

    return <>
        { grouped }
    </>
}

function CategoryView(
    category: string,
    offers: Product[],
    openProduct: (uuid: Uuid) => void,
): ReactElement {
    let elements = offers
        .filter(x => x.category === category)
        .map(x => ProductCard(x, openProduct))

    return <div key={category}>
        <Title
            order={2}
            mt='xs'
        >
            { category }
        </Title>

        <SimpleGrid cols={{
            base: 1,
            sm: 3,
        }}>
            { elements }
        </SimpleGrid>
    </div>
}

function ProductCard(
    product: Product,
    openProduct: (uuid: Uuid) => void,
): ReactElement {
    return (
        <Card key={product.uuid}>
            <Flex>
                <Image
                    src={`https://images.evetech.net/types/${product.image_type_id}/${product.image_type}?size=128`}
                    h={128}
                    w={128}
                />

                <Stack
                    gap="xs"
                    style={{
                        marginLeft: '10px',
                        width: '100%'
                    }}
                >
                    <Title order={4}>{ product.name }</Title>
                    <Text>Price: { formatNumber(product.price) }</Text>
                    <div>
                    {
                        product
                            .tags
                            .map(x => <Pill ml="xs" key={x}>{x}</Pill>)
                    }
                    </div>

                    <Button
                        fullWidth
                        onClick={ () => openProduct(product.uuid) }
                    >
                        Configure
                    </Button>
                </Stack>
            </Flex>
        </Card>
    );
}

export type Filter = {
    name?: string,
    tags?: string[],
}
