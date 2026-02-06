import { Alert } from "@mantine/core";
import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { createProduct } from "@/services/product/create";
import { LIST_PRODUCT } from "@/services/product/list";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useState } from "react";
import { Route as UpdateProductComponentRoute } from '@/routes/admin/products_/$productUuid.index';
import { ProductFormComponent, type ProductForm } from "../-components/productForm";

export const Route = createFileRoute('/admin/products/create')({
    component: AdminCreateProductsComponent,
});

function AdminCreateProductsComponent() {
    const navigation = useNavigate({ from: Route.fullPath });

    const [errorCreate, setErrorCreate] = useState<string | undefined>();

    const queryClient = useQueryClient();
    const create = useMutation({
        mutationFn: async (data: ProductForm) => {
            return await createProduct(data);
        },
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: [LIST_PRODUCT] })
        },
    });

    const product: ProductForm = {
        name: '',
        description: '',
        price: undefined,
        image_type: 'icon',
        image_type_id: 56085,
        category: 'Uncategorized',
        tags: [],
        additional_options: [],
        content: '',
    };

    const onSubmit = async (value: ProductForm) => {
        await create
            .mutateAsync(value)
            .then(x => {
                navigation({
                    to: UpdateProductComponentRoute.to,
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
            })
    };

    const notification = () => {
        if (errorCreate) {
            return <Alert
                variant='light'
                color='red'
                title='Update error'
                onClose={ () => setErrorCreate(undefined) }
                withCloseButton
            >
                There was an error while creating. Please try again later.
            </Alert>;
        }
    };

    return <>
        { notification() }

        {
            ProductFormComponent({
                product,
                onSubmit,
            })
        }
    </>
}
