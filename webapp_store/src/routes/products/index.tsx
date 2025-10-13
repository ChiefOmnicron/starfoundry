import { Alert, Button, Card, Flex, Image, Pill, SimpleGrid, Stack, Text, Title } from '@mantine/core';
import { createFileRoute, useNavigate } from '@tanstack/react-router';
import { formatNumber, type Uuid } from '@/services/utils';
import { LoadingAnimation } from '@/components/LoadingAnimation';
import { LoadingError } from '@/components/LoadingError';
import { useListCategory } from '@/services/product/category_list';
import { useListProducts, type Product, type ProductFilter } from '@/services/product/list';
import { Filter, type FilterPropEntry, type SelectedFilter } from '@/components/Filter';
import { Route as ProductOrderRoute } from '@/routes/products_/$productUuid.index';
import { useState, type ReactElement } from 'react';

export const Route = createFileRoute('/products/')({
    beforeLoad: async ({ context }) => {
        if (!await context.auth.isAuthenticated()) {
            throw context.auth.login();
        }
    },
    component: StoreComponent,
});

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

const tags = [
    { key: "Amarr", label: "Amarr" },
    { key: "Caldari", label: "Caldari" },
    { key: "Gallente", label: "Gallente" },
    { key: "Minmatar", label: "Minmatar" },
    { key: "Triglavian", label: "Triglavian" },
    { key: "Upwell", label: "Upwell" },
    { key: "ORE", label: "ORE" },

    { key: "Dreadnought", label: "Dreadnought" },
    { key: "Faction Dreadnought", label: "Faction Dreadnought" },
    { key: "Lancer", label: "Lancer" },
    { key: "Carrier", label: "Carrier" },
    { key: "FAX", label: "FAX" },
    { key: "Super Carrier", label: "Super Carrier" },
    { key: "Titan", label: "Titan" },
    { key: "Freighter", label: "Freighter" },
    { key: "Jump Freighter", label: "Jump Freighter" },
    { key: "Pirate Dreadnought", label: "Pirate Dreadnought" },
    { key: "Pirate FAX", label: "Pirate FAX" },
    { key: "Pirate Super Carrier", label: "Pirate Super Carrier" },
    { key: "Pirate Titan", label: "Pirate Titan" },

    { key: "Fit", label: "Fit" },
    { key: "Hull", label: "Hull" },
    { key: "Beehive", label: "Beehive" },
    { key: "Brave", label: "Brave" },
    { key: "GSF", label: "GSF" },

    { key: "ARX", label: "ARX" },
    { key: "Active", label: "Active" },
    { key: "Buffer", label: "Buffer" },
    { key: "HAW", label: "HAW" },
    { key: "LR", label: "LR" },
    { key: "HAW Refit", label: "HAW Refit" },
    { key: "LR Refit", label: "LR Refit" },

    { key: "Moros", label: "Moros" },
    { key: "Naglfar", label: "Naglfar" },
    { key: "Phoenix", label: "Phoenix" },
    { key: "Revelation", label: "Revelation" },
    
    { key: "Moros Navy Issue", label: "Moros Navy Issue" },
    { key: "Naglfar Fleet Issue", label: "Naglfar Fleet Issue" },
    { key: "Phoenix Navy Issue", label: "Phoenix Navy Issue" },
    { key: "Revelation Navy Issue", label: "Revelation Navy Issue" },

    { key: "Archon", label: "Archon" },
    { key: "Chimera", label: "Chimera" },
    { key: "Nidhoggur", label: "Nidhoggur" },
    { key: "Thanatos", label: "Thanatos" },

    { key: "Apostle", label: "Apostle" },
    { key: "Lif", label: "Lif" },
    { key: "Minokawa", label: "Minokawa" },
    { key: "Ninazu", label: "Ninazu" },
    
    { key: "Rorqual", label: "Rorqual" },

    { key: "Bane", label: "Bane" },
    { key: "Hubris", label: "Hubris" },
    { key: "Karura", label: "Karura" },
    { key: "Valravn", label: "Valravn" },

    { key: "Aeon", label: "Aeon" },
    { key: "Hel", label: "Hel" },
    { key: "Nyx", label: "Nyx" },
    { key: "Wyvern", label: "Wyvern" },

    { key: "Avatar", label: "Avatar" },
    { key: "Erebus", label: "Erebus" },
    { key: "Leviathan", label: "Leviathan" },
    { key: "Ragnarok", label: "Ragnarok" },

    { key: "Avalanche", label: "Avalanche" },
    { key: "Charon", label: "Charon" },
    { key: "Fenrir", label: "Fenrir" },
    { key: "Obelisk", label: "Obelisk" },
    { key: "Providence", label: "Providence" },

    { key: "Anshar", label: "Anshar" },
    { key: "Ark", label: "Ark" },
    { key: "Nomad", label: "Nomad" },
    { key: "Rhea", label: "Rhea" },

    { key: "Caiman", label: "Caiman" },
    { key: "Chemosh", label: "Chemosh" },
    { key: "Sarathiel", label: "Sarathiel" },

    { key: "Dagon", label: "Dagon" },
    { key: "Loggerhead", label: "Loggerhead" },

    { key: "Revenant", label: "Revenant" },
    { key: "Vehement", label: "Vehement" },

    { key: "Azariel", label: "Azariel" },
    { key: "Komodo", label: "Komodo" },
    { key: "Molok", label: "Molok" },
    { key: "Vanquisher", label: "Vanquisher" },
    { key: "Zirnitra", label: "Zirnitra" },
];

function StoreComponent() {
    const navigation = useNavigate();
    const [filterParams, setFilterParams] = useState<ProductFilter>({});

    const {
        isPending: isPendingCategory,
        isError: isErrorCategory,
        data: categories,
    } = useListCategory();

    const {
        isPending: isPendingProducts,
        isError: isErrorProducts,
        data: products,
    } = useListProducts(filterParams);

    if (isPendingCategory || isPendingProducts) {
        return LoadingAnimation();
    }
    if (isErrorCategory || isErrorProducts) {
        return LoadingError();
    }

    const filterOptions: FilterPropEntry[] = [{
        label: 'Name',
        key: 'name',
        type: 'STRING',
    }, {
        label: 'Category',
        key: 'category',
        type: 'SELECT',
        options: (categories || []).map(x => {
            return {
                label: x,
                key: x,
            }
        })
    }, {
        label: 'Tag',
        key: 'tag',
        type: 'MULTISELECT',
        options: tags
            .filter(x => {
                if (!filterParams.tags) {
                    return true;
                }

                return filterParams.tags?.indexOf(x.label) === -1;
            }),
    }];
    const filterChange = (filters: SelectedFilter[]) => {
        const tags = filters.filter(x => x.key === 'tag')?.flatMap(x => x.value as string);

        setFilterParams({
            name: filters.find(x => x.key === 'name')?.value as string,
            category: filters.find(x => x.key === 'category')?.value as string,
            tags: tags.length > 0 ? tags.join(',') : undefined,
        });
    };

    const navigateProduct = (id: Uuid) => {
        navigation({
            to: ProductOrderRoute.to,
            params: {
                productUuid: id,
            }
        });
    }

    const grouped =  categoryOrder
        .map(x => CategoryView(x, products, navigateProduct));

    return <>
        <Alert
            variant='light'
            color='blue'
            title='Note'
            style={{
                marginBottom: '5px'
            }}
        >
            Please note: The prices are calculated daily. The price you order at is locked in, and will not be adjusted to the latest store price when delivered.
        </Alert>

        <Filter
            entries={filterOptions}
            onFilterChange={filterChange}
        />

        { grouped }
    </>
}

function CategoryView(
    category: string,
    products: Product[],
    openProduct: (id: Uuid) => void,
) {
    let elements = (products || [])
        .filter(x => x.category === category)
        .map(x => ProductCard(x, openProduct))

    if (elements.length === 0) {
        return;
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
    const postFix = () => {
        if (
            product.category === 'Pirate Dreadnought' ||
            product.category === 'Pirate FAX' ||
            product.category === 'Pirate Super Carrier' ||
            product.category === 'Pirate Titan'
        ) {
            return ' + BPC'
        }
    }

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
                    <Text>Price: { formatNumber(product.price) }{ postFix() }</Text>
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
