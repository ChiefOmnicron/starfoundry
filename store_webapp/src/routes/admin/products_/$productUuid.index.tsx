import { LoadingAnimation } from '@/components/LoadingAnimation';
import { LoadingError } from '@/components/LoadingError';
import { useFetchProduct } from '@/services/product/fetch';
import { Alert } from '@mantine/core';
import { createFileRoute } from '@tanstack/react-router'
import { ProductFormComponent, type ProductForm } from '../-components/productForm';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { updateProduct } from '@/services/product/update';
import { LIST_PRODUCT } from '@/services/product/list';
import { useState } from 'react';

type QueryParams = {
    created?: boolean;
}

export const Route = createFileRoute('/admin/products_/$productUuid/')({
    component: UpdateProductComponent,
    validateSearch: (params: {
        created: boolean,
    }): QueryParams => {
        return {
            created: (params.created) || undefined
        };
    }
})

function UpdateProductComponent() {
    const { created: createdResource } = Route.useSearch();
    const { productUuid } = Route.useParams();

    const [successfulUpdate, setSuccessfulUpdated] = useState<boolean>();
    const [errorUpdate, setErrorUpdated] = useState<string | undefined>();

    const queryClient = useQueryClient();
    const update = useMutation({
        mutationFn: async (data: ProductForm) => {
            return await updateProduct(productUuid, data);
        },
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: [LIST_PRODUCT] })
            setErrorUpdated(undefined);
            setSuccessfulUpdated(true);
        },
    });

    const {
        isPending,
        isError,
        data,
    } = useFetchProduct(productUuid);

    if (isPending) {
        return LoadingAnimation();
    }

    if (isError) {
        return LoadingError();
    }

    const onSubmit = async (value: ProductForm) => {
        await update
            .mutateAsync(value)
            .catch(error => {
                setErrorUpdated(error);
                setSuccessfulUpdated(false);
            })
    };

    const notification = () => {
        if (successfulUpdate) {
            return <Alert
                mt="sm"
                variant='light'
                color='green'
                title='Update successful'
                data-cy="successfulUpdate"
                onClose={ () => setSuccessfulUpdated(false) }
                withCloseButton
            >
                The product was updated
            </Alert>;
        } else if (createdResource) {
            return <Alert
                mt="sm"
                variant='light'
                color='green'
                title='Create successful'
                data-cy="createSuccessful"
            >
                The product was successfully created
            </Alert>;
        } else if (errorUpdate) {
            return <Alert
                mt="sm"
                variant='light'
                color='red'
                title='Update error'
                data-cy="errorUpdate"
                onClose={ () => setErrorUpdated(undefined) }
                withCloseButton
            >
                There was an error while updating. Please try again later.
            </Alert>;
        }
    }

    const product = (): ProductForm => {
        if (!data) {
            return {} as any;
        } else {
            const items = data
                .content
                .map(x => `${x.name}\t${x.quantity}`)
                .join('\n');
            return {
                ...data,
                content: items
            }
        }
    }

    return <>
        { notification() }

        {
            ProductFormComponent({
                product: product(),
                onSubmit
            })
        }
    </>
}
