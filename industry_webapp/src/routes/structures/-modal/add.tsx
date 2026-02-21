import { Alert, Button, Flex, Stack, Table } from '@mantine/core';
import { createStructure, type CreateStructure } from '@starfoundry/components/services/structure/create';
import { Route as StructureRoute } from '@/routes/structures_/$structureId.index';
import { useMutation } from '@tanstack/react-query';
import { useNavigate } from '@tanstack/react-router';
import { useState } from 'react';
import type { ResolveStructureResponse } from '@starfoundry/components/services/structure/resolveStructure';
import type { TypeId, Uuid } from '@starfoundry/components/services/utils';
import { Dotlan } from '@starfoundry/components/misc/Dotlan';
import { RigSelector } from '@starfoundry/components/selectors/RigSelector';
import { ServiceSelector } from '@starfoundry/components/selectors/ServiceSelector';
import { LoadingAnimation } from '@starfoundry/components/misc/LoadingAnimation';
import { ResolveStructure } from '@starfoundry/components/structure/ResolveStructure';

export function AddStructure({
    close
}: Props) {
    const navigation = useNavigate();

    const [errorCreate, setErrorCreate] = useState<string | undefined>();
    const [errorResolveStructure, setErrorResolveStructure] = useState<string | undefined>();

    const [isLoading, setIsLoading] = useState<boolean>(false);

    const [resolvedStructure, setResolvedStructure] = useState<ResolveStructureResponse | undefined>();
    const [selectedRigs, setSelectedRigs] = useState<TypeId[]>([]);
    const [selectedServices, setSelectedServices] = useState<(TypeId | null)[]>([]);

    const mutation = useMutation({
        mutationFn: async (value: CreateStructure) => {
            return await createStructure(value)
        },
        onSuccess: (data: { id: Uuid }) => {
            return navigation({
                to: StructureRoute.to,
                params: {
                    structureId: data.id,
                },
                search: {
                    created: true,
                },
            });
        },
        onError: (error) => {
            setErrorCreate(error.message);
        }
    });

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
            <Table data-cy="infoTable">
                <Table.Tbody>
                    <Table.Tr>
                        <Table.Th>Name</Table.Th>
                        <Table.Td>{ resolvedStructure.name }</Table.Td>
                    </Table.Tr>
                    <Table.Tr>
                        <Table.Th>Type</Table.Th>
                        <Table.Td>{ resolvedStructure.item.name }</Table.Td>
                    </Table.Tr>
                    <Table.Tr>
                        <Table.Th>System</Table.Th>
                        <Table.Td>
                            <Dotlan
                                system={resolvedStructure.system}
                            />
                        </Table.Td>
                    </Table.Tr>
                </Table.Tbody>
            </Table>

            <RigSelector
                rigs={resolvedStructure.installable_rigs}
                selected={selectedRigs}
                onSelect={(selected: TypeId[]) => {
                    setSelectedRigs(selected);
                }}
            />

            <ServiceSelector
                services={resolvedStructure.installable_services}
                onSelect={(selected: TypeId[]) => {
                    setSelectedServices(selected)
                }}
            />
        </>
    }

    const footerButtons = () => {
        return <>
            <Flex
                justify="flex-end"
                gap="sm"
            >
                <Button
                    data-cy="closeStructure"
                    mt="sm"
                    variant="subtle"
                    color="gray"
                    onClick={close}
                >
                    Close
                </Button>

                {
                    !resolvedStructure
                        ?   <></>
                        :   <Button
                                data-cy="addStructure"
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
                                        position:           resolvedStructure.position,
                                    });
                                }}
                            >
                                Add
                            </Button>
                }
            </Flex>
        </>
    }

    return <>
        { notification() }

        { isLoading ? LoadingAnimation() : <></> }

        <Stack>
            <ResolveStructure
                onError={(error) => {
                    setErrorResolveStructure(error.message);
                }}
                onSuccess={(structure) => {
                    setErrorResolveStructure(undefined);
                    setResolvedStructure(structure)
                }}
                onLoad={(loading: boolean) => {
                    setIsLoading(loading)
                }}
            />

            <Stack>
                { structureDetailInformation() }
            </Stack>

            { footerButtons() }
        </Stack>
    </>
}

export type Props = {
    close: () => void,
}
