import { LoadingAnimation } from '@/components/LoadingAnimation';
import { LoadingError } from '@/components/LoadingError';
import { useListProducts, type Product } from '@/services/product/list';
import { Alert, Button, Center, Flex, NumberFormatter, Pill, Stack, Table, Title, UnstyledButton } from '@mantine/core';
import { createFileRoute, Link, useNavigate } from '@tanstack/react-router';
import { createColumnHelper, flexRender, getCoreRowModel, useReactTable } from '@tanstack/react-table';
import { Route as RouteCreateProductRoute } from '@/routes/admin/products/create';

interface QueryParams {
    deleted?: boolean;
}

export const Route = createFileRoute('/admin/products/')({
    component: AdminProductsComponent,
    validateSearch: (params: {
        deleted: boolean,
    }): QueryParams => {
        return {
            deleted: (params.deleted) || undefined
        };
    }
});

const columnHelper = createColumnHelper<Product>();
const columns = [
    columnHelper.accessor('name', {
        id: 'name',
        cell: info => <UnstyledButton
            component={Link}
            to={
                `/admin/products/${info.row.original.uuid}`
            }
            style={{
                color: 'var(--mantine-color-blue-4)',
                fontSize: 'var(--mantine-font-size-sm)'
            }}
        >
            { info.getValue() }
        </UnstyledButton>,
        header: () => 'Name',
    }),
    columnHelper.accessor('price', {
        id: 'Price',
        cell: info => {
            return <NumberFormatter
                value={info.getValue()}
                thousandSeparator
            />
        },
        header: () => 'Price',
    }),
    columnHelper.accessor('category', {
        id: 'category',
        cell: info => info.getValue(),
        header: () => 'Category',
    }),
    columnHelper.accessor('tags', {
        id: 'tags',
        cell: info => {
            if (info.getValue()) {
                return info
                    .getValue()
                    .map(x => <Pill ml="xs" key={x}>{ x }</Pill>)
            }
        },
        header: () => 'Tags',
    }),
];

function AdminProductsComponent() {
    const navigation = useNavigate({ from: Route.fullPath });
    const { deleted: deletedResource } = Route.useSearch();

    const {
        isPending,
        isError,
        isFetching,
        data: products,
    } = useListProducts();

    const table = useReactTable<Product>({
        columns: columns,
        data: products,
        autoResetPageIndex: false,
        getCoreRowModel: getCoreRowModel(),
    });

    const navigationCreateComponent = () => {
        navigation({ to: RouteCreateProductRoute.to });
    }

    const notification = () => {
        if (deletedResource) {
            return <Alert
                mt="sm"
                variant='light'
                color='green'
                title='Delete successful'
                data-cy="deleteSuccessful"
            >
                The project group was successfully deleted
            </Alert>;
        }
    }

    const dataTable = () => {
        return <>
            <Flex
                align='center'
                justify='flex-start'
                direction='row-reverse'
                pb='sm'
            >
                <Button
                    variant='filled'
                    onClick={ () => navigationCreateComponent() }
                >
                    Create Product
                </Button>
            </Flex>

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
        } else if (products.length > 0) {
            return dataTable();
        } else {
            return <>
                <Center mt={50} data-cy="noData">
                    <Stack>
                        <Title order={4}>No products yet</Title>

                        <Button
                            variant='filled'
                            onClick={ () => navigationCreateComponent() }
                        >
                            Create Product
                        </Button>
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
