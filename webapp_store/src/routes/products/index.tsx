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

function StoreComponent() {
    const navigation = useNavigate();

    const {
        isPending: isPendingCategory,
        isError: isErrorCategory,
        data: categories
    } = useListCategory();
    console.log(categories)

    const {
        isPending: isPendingProducts,
        isError: isErrorProducts,
        data: products,
    } = useListProducts();

    if (isPendingCategory || isPendingProducts) {
        return LoadingAnimation();
    }
    if (isErrorCategory || isErrorProducts) {
        return LoadingError();
    }

    const navigateProduct = (id: Uuid) => {
        navigation({
            to: ProductOrderRoute.to,
            params: {
                productUuid: id,
            }
        });
    }

    const categoryOrder = [
        'Faction Dreadnought',
        'Dreadnought',
        'FAX',
        'Carrier',
        'Lancer',

        'Brave Fits - Buffer',
        'Brave Fits - Buffer LR',
        'Brave Fits - Carrier',
        'Brave Fits - ARX',
        'Brave Fits - Active',
        'Brave Fits - Active HAW Refit',
        'Brave Fits - Buffer LR Refits',
        'GSF Fits - Beehive',

        'Super Carrier',
        'Brave Fits - Super Carrier',
        'Titan',
        'Brave Fits - Titan',

        'Freighter',
        'Jump Freighter',
        'Misc',

        'Pirate Dreadnought',
        'Pirate FAX',
        'Pirate Super Carrier',
        'Pirate Titan',
    ];

    const grouped =  categoryOrder
        .map(x => CategoryView(x, products, navigateProduct));

    return <>
        { grouped }
    </>
}

function CategoryView(
    category: string,
    products: Product[],
    openProduct: (id: Uuid) => void,
): ReactElement {
    let elements = (products || [])
        .filter(x => x.category === category)
        .map(x => ProductCard(x, openProduct))

    if (elements.length === 0) {
        console.log(category);
        return <></>
    }

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
        <Card key={product.id}>
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
                        onClick={ () => openProduct(product.id) }
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
