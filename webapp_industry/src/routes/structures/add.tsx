import { Alert, Button, Flex, Stack, TextInput, Title } from '@mantine/core';
import { createFileRoute, useNavigate } from '@tanstack/react-router'
import { ResolveStructure } from '@/routes/structures/_components/ResolveStructure';
import { RigSelectionGroup } from '@/routes/structures/_components/RigSelectionGroup';
import { Route as StructureOverviewRoute } from '@/routes/structures/index';
import { ServiceSelectionGroup } from '@/routes/structures/_components/ServiceSelectionGroup';
import { type CreateProjectGroup as AddStructure } from '@/services/project-group/create';
import { type ResolveStructureResponse } from '@/services/structure/resolveStructure';
import { type TypeId } from '@/services/utils';
import { useState } from 'react';
import LoadingAnimation from '@/components/LoadingAnimation';
import { useMutation } from '@tanstack/react-query';
import { createStructure, type CreateStructure } from '@/services/structure/create';

export const Route = createFileRoute('/structures/add')({
    component: AddStructure,
})

function AddStructure() {
    const navigation = useNavigate({ from: Route.fullPath });

    const [errorCreate, setErrorCreate] = useState<string | undefined>();
    const [errorResolveStructure, setErrorResolveStructure] = useState<string | undefined>();

    const [isLoading, setIsLoading] = useState<boolean>(false);

    const [resolvedStructure, setResolvedStructure] = useState<ResolveStructureResponse | undefined>();
    const [selectedRigs, setSelectedRigs] = useState<(TypeId | null)[]>([]);
    const [selectedServices, setSelectedServices] = useState<(TypeId | null)[]>([]);

    const mutation = useMutation({
        mutationFn: async (value: CreateStructure) => {
            console.log(value);
            //return axios.post('/todos', newTodo)
            return await createStructure(value)
        },
    })


    const notification = () => {
        if (errorCreate) {
            return <Alert
                mt="sm"
                variant='light'
                color='red'
                title='Create error'
                data-cy="errorCreate"
                onClose={ () => setErrorCreate(undefined) }
                withCloseButton
            >
                There was an error while creating. Please try again later.
            </Alert>;
        } else if (errorResolveStructure) {
            return <Alert
                mt="sm"
                variant='light'
                color='red'
                title='Resolve structure error'
                data-cy="errorResolveStructure"
                onClose={ () => setErrorResolveStructure(undefined) }
                withCloseButton
            >
                There was an error while resolving the structure. Please validate the input.
            </Alert>;
        }
    };

    const structureDetailInformation = () => {
        if (!resolvedStructure) {
            return <></>
        }

        return <>
            <TextInput
                label="Name"
                id="name"
                name="name"
                value={resolvedStructure.name}
                disabled
            />

            <TextInput
                label="Structure Type"
                id="structure_type"
                name="structure_type"
                value={resolvedStructure.item.name}
                disabled
            />

            <TextInput
                label="System"
                id="system"
                name="system"
                value={resolvedStructure.system.system_name}
                disabled
            />

            <Title order={2}>Installed Rigs</Title>
            <RigSelectionGroup
                rigs={resolvedStructure.rigs}
                onSelect={(selected) => {
                    setSelectedRigs(selected)
                }}
            />

            <Title order={2}>Installed Services</Title>
            <ServiceSelectionGroup
                services={resolvedStructure.services}
                onSelect={(selected) => {
                    setSelectedServices(selected)
                }}
            />
        </>
    }

    const footerButtons = () => {
        if (!resolvedStructure) {
            return <>
                <Flex
                    justify="flex-end"
                    gap="sm"
                >
                    <Button
                        mt="sm"
                        variant="subtle"
                        color="gray"
                        onClick={() => navigation({ to: StructureOverviewRoute.to })}
                    >
                        Back
                    </Button>
                </Flex>
            </>
        }

        return <Flex
                justify="flex-end"
                gap="sm"
            >
                <Button
                    mt="sm"
                    variant="subtle"
                    color="gray"
                    onClick={() => navigation({ to: StructureOverviewRoute.to })}
                >
                    Back
                </Button>
                <Button
                    data-cy="add"
                    mt="sm"
                    type="submit"
                    onClick={() => {
                        mutation.mutate({
                            name:               resolvedStructure.name,
                            rigs:               selectedRigs.filter(x => x) as TypeId[],
                            services:           selectedServices.filter(x => x) as TypeId[],
                            structure_id:       resolvedStructure.structure_id,
                            structure_type_id:  resolvedStructure.item.type_id,
                            system_id:          resolvedStructure.system.system_id,
                        });
                    }}
                >
                    Add
                </Button>
            </Flex>
    }

    return <>
        { notification() }

        { isLoading ? LoadingAnimation() : <></> }

        <Stack>
            <Title order={2}>General Information</Title>

            <ResolveStructure
                onError={(error) => {
                    setErrorResolveStructure(error.message);
                }}
                onSuccess={(structure) => {
                    setErrorResolveStructure(undefined);
                    setResolvedStructure(structure)
                }}
                onLoad={(loading: boolean) => setIsLoading(loading)}
            />

            <Stack>
                { structureDetailInformation() }
            </Stack>

            { footerButtons() }
        </Stack>
    </>
}
