import { Button, Grid, Group, InputBase, InputWrapper, NumberInput, SegmentedControl, Stack } from "@mantine/core";
import { InputWrapperMarginBottom } from "@internal/misc/InputWrapperMarginBottom";
import { ROUTE_TYPE_HAULING_ROUTE, ROUTE_TYPE_HAULING_SERVICE, ROUTE_TYPE_JUMP_ROUTE, ROUTE_TYPES, type RouteType } from "./services/options";
import { StructureCard } from "@internal/cards/StructureCard";
import { StructureSelectorModal } from "@internal/selectors/StructureSelectorModal";
import { useDisclosure } from "@mantine/hooks";
import { useForm } from "@tanstack/react-form";
import { useListStructure, type Structure } from "@internal/services/structure/list";
import type { ReactElement } from "react";
import type { Uuid } from "@internal/services/utils";

export function CreateRoute(): ReactElement {
    const [startStructureSelectorOpened, { open: openStartStructureSelector, close: closeStartStructureSelectorClose }] = useDisclosure(false);
    const [endStructureSelectorOpened, { open: openEndStructureSelector, close: closeEndStructureSelectorClose }] = useDisclosure(false);

    const {
        data: structures,
    } = useListStructure({
        include_npc: true,
    });

    const form = useForm({
        defaultValues: {
            name:                       '',
            typ:                        ROUTE_TYPE_JUMP_ROUTE.value,

            start_structure:            '',
            end_structure:              '',

            // for route type ROUTE
            jump_route: {
                fuel_usage:                 0,
            },

            // for route type HAULING_SERVICE
            hauling_route: {
                max_cargo_m3:               350_000,
                fuel_usage:                 0,
            },

            // for route type HAULING_SERVICE
            hauling_service: {
                contract_to:                '',
                price_per_m3:               0,
                max_cargo_m3:               350_000,
                collateral_percent:         10,
            },
        },
        onSubmit: async (entry) => {
            console.log(entry);
        }
    });

    const structureCard = (id: Uuid) => {
        const structure = (structures || []).find(x => x.id === id);
        if (structure) {
            return <StructureCard
                structure={structure}
            />;
        } else {
            return <></>;
        }
    }

    const getStructure = (id: Uuid): Structure[] => {
        const structure = (structures || []).find(x => x.id === id);
        if (structure) {
            return [structure];
        } else {
            return [];
        }
    }

    return <>
        <form
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
                            <InputBase
                                data-1p-ignore
                                withAsterisk
                                data-cy="name"
                                label="Name"
                                description="Name of the Route"
                                placeholder="My cool route"
                                id={field.name}
                                name={field.name}
                                value={field.state.value}
                                error={
                                    !field.state.meta.isValid && field.state.meta.errors.join(', ')
                                }
                                onBlur={field.handleBlur}
                                onChange={(e) => {
                                    field.handleChange(e.target.value);
                                }}
                            />
                        </>
                    }}
                />

                <form.Field
                    name="start_structure"
                    children={(field) => {
                        return <>
                            <InputWrapper
                                label="Start Structure"
                                description="Structure the route starts at"
                                withAsterisk
                            >
                                <InputWrapperMarginBottom />

                                <Stack>
                                    <Group>
                                        <StructureSelectorModal
                                            opened={startStructureSelectorOpened}
                                            onClose={closeStartStructureSelectorClose}

                                            onSelect={(structure) => {
                                                if (structure[0]) {
                                                    field.handleChange(structure[0].id);
                                                    closeStartStructureSelectorClose();
                                                }
                                            }}
                                            structures={structures || []}
                                            selected={getStructure(field.state.value)}
                                        />

                                        <Button
                                            onClick={() => openStartStructureSelector()}
                                        >
                                            Set structure
                                        </Button>
                                    </Group>

                                    <Grid>
                                        <Grid.Col
                                            span={6}
                                        >
                                            {structureCard(field.state.value)}
                                        </Grid.Col>
                                    </Grid>
                                </Stack>
                            </InputWrapper>
                        </>
                    }}
                />

                <form.Field
                    name="end_structure"
                    children={(field) => {
                        return <>
                            <InputWrapper
                                label="End Structure"
                                description="Structure the route ends at"
                                withAsterisk
                            >
                                <InputWrapperMarginBottom />

                                <Stack>
                                    <Group>
                                        <StructureSelectorModal
                                            opened={endStructureSelectorOpened}
                                            onClose={closeEndStructureSelectorClose}

                                            onSelect={(structure) => {
                                                if (structure[0]) {
                                                    field.handleChange(structure[0].id);
                                                    closeEndStructureSelectorClose();
                                                }
                                            }}
                                            structures={structures || []}
                                            selected={getStructure(field.state.value)}
                                        />

                                        <Button
                                            onClick={() => openEndStructureSelector()}
                                        >
                                            Set structure
                                        </Button>
                                    </Group>

                                    <Grid>
                                        <Grid.Col
                                            span={6}
                                        >
                                            {structureCard(field.state.value)}
                                        </Grid.Col>
                                    </Grid>
                                </Stack>
                            </InputWrapper>
                        </>
                    }}
                />

                <form.Field
                    name="typ"
                    children={(field) => {
                        return <>
                            <InputWrapper
                                label="Type"
                                description="Select the type of the Route"
                                withAsterisk
                            >
                                <InputWrapperMarginBottom />

                                <Group>
                                    <SegmentedControl
                                        data-cy="typ"
                                        id={field.name}
                                        name={field.name}
                                        value={field.state.value}
                                        data={ROUTE_TYPES}
                                        onChange={(e) => field.handleChange(e as RouteType)}
                                    />
                                </Group>
                            </InputWrapper>
                        </>
                    }}
                />

                                <form.Subscribe
                    selector={(state) => [state.values.typ]}
                    children={([typ]) => {
                        if (typ !== ROUTE_TYPE_JUMP_ROUTE.value) {
                            return <></>;
                        }

                        return <>
                            <form.Field
                                name="route.fuel_usage"
                                children={(field) => {
                                    return <>
                                        <NumberInput
                                            data-1p-ignore
                                            thousandSeparator
                                            withAsterisk
                                            data-cy="fuel_usage"
                                            label="Fuel usage"
                                            description="Isotopes used for the whole journey"
                                            id={field.name}
                                            name={field.name}
                                            value={field.state.value}
                                            error={
                                                !field.state.meta.isValid && field.state.meta.errors.join(', ')
                                            }
                                            onBlur={field.handleBlur}
                                            min={0}
                                            onChange={(e) => {
                                                if (e) {
                                                    field.handleChange(e as number);
                                                }
                                            }}
                                        />
                                    </>
                                }}
                            />
                        </>
                    }}
                />

                <form.Subscribe
                    selector={(state) => [state.values.typ]}
                    children={([typ]) => {
                        if (typ !== ROUTE_TYPE_HAULING_ROUTE.value) {
                            return <></>;
                        }

                        return <>
                            <form.Field
                                name="hauling_route.fuel_usage"
                                children={(field) => {
                                    return <>
                                        <NumberInput
                                            data-1p-ignore
                                            thousandSeparator
                                            withAsterisk
                                            data-cy="fuel_usage"
                                            label="Fuel usage"
                                            description="Isotopes used for the whole journey"
                                            id={field.name}
                                            name={field.name}
                                            value={field.state.value}
                                            error={
                                                !field.state.meta.isValid && field.state.meta.errors.join(', ')
                                            }
                                            onBlur={field.handleBlur}
                                            min={0}
                                            onChange={(e) => {
                                                if (e) {
                                                    field.handleChange(e as number);
                                                }
                                            }}
                                        />
                                    </>
                                }}
                            />

                            <form.Field
                                name="hauling_route.max_cargo_m3"
                                children={(field) => {
                                    return <>
                                        <NumberInput
                                            data-1p-ignore
                                            thousandSeparator
                                            withAsterisk
                                            data-cy="max_cargo_m3"
                                            label="Max m3 cargo"
                                            description="The maximum cargo size that is allowed"
                                            suffix=" m3"
                                            id={field.name}
                                            name={field.name}
                                            value={field.state.value}
                                            error={
                                                !field.state.meta.isValid && field.state.meta.errors.join(', ')
                                            }
                                            onBlur={field.handleBlur}
                                            onChange={(e) => {
                                                if (e) {
                                                    field.handleChange(e as number);
                                                }
                                            }}
                                        />
                                    </>
                                }}
                            />
                        </>
                    }}
                />

                <form.Subscribe
                    selector={(state) => [state.values.typ]}
                    children={([typ]) => {
                        if (typ !== ROUTE_TYPE_HAULING_SERVICE.value) {
                            return <></>;
                        }

                        return <>
                            <form.Field
                                name="hauling_service.contract_to"
                                validators={{
                                    onBlur: ({ value }) => {
                                        return (value.trimStart().length === 0 ? 'The field is required' : undefined) ||
                                            (value.length > 100 ? 'Maximum allowed chars is 100' : undefined)
                                    }
                                }}
                                children={(field) => {
                                    return <>
                                        <InputBase
                                            data-1p-ignore
                                            withAsterisk
                                            data-cy="contract_to"
                                            label="Contract to"
                                            description="Name of the character/corporation to set as the receiving entity"
                                            placeholder="Some Corporation Name"
                                            id={field.name}
                                            name={field.name}
                                            value={field.state.value}
                                            error={
                                                !field.state.meta.isValid && field.state.meta.errors.join(', ')
                                            }
                                            onBlur={field.handleBlur}
                                            onChange={(e) => {
                                                field.handleChange(e.target.value);
                                            }}
                                        />
                                    </>
                                }}
                            />

                            <form.Field
                                name="hauling_service.price_per_m3"
                                children={(field) => {
                                    return <>
                                        <NumberInput
                                            data-1p-ignore
                                            thousandSeparator
                                            withAsterisk
                                            data-cy="price_per_m3"
                                            label="Price per m3"
                                            description="The isk/m3 for calculating the price"
                                            suffix=" ISK/m3"
                                            id={field.name}
                                            name={field.name}
                                            value={field.state.value}
                                            error={
                                                !field.state.meta.isValid && field.state.meta.errors.join(', ')
                                            }
                                            onBlur={field.handleBlur}
                                            onChange={(e) => {
                                                if (e) {
                                                    field.handleChange(e as number);
                                                }
                                            }}
                                        />
                                    </>
                                }}
                            />

                            <form.Field
                                name="hauling_service.max_cargo_m3"
                                children={(field) => {
                                    return <>
                                        <NumberInput
                                            data-1p-ignore
                                            thousandSeparator
                                            withAsterisk
                                            data-cy="max_cargo_m3"
                                            label="Max m3 cargo"
                                            description="The maximum cargo size that is allowed"
                                            suffix=" m3"
                                            id={field.name}
                                            name={field.name}
                                            value={field.state.value}
                                            error={
                                                !field.state.meta.isValid && field.state.meta.errors.join(', ')
                                            }
                                            onBlur={field.handleBlur}
                                            onChange={(e) => {
                                                if (e) {
                                                    field.handleChange(e as number);
                                                }
                                            }}
                                        />
                                    </>
                                }}
                            />

                            <form.Field
                                name="hauling_service.collateral_percent"
                                children={(field) => {
                                    return <>
                                        <NumberInput
                                            data-1p-ignore
                                            thousandSeparator
                                            withAsterisk
                                            data-cy="collateral"
                                            label="Collateral"
                                            description="Collateral % that should be used"
                                            suffix=" %"
                                            id={field.name}
                                            name={field.name}
                                            value={field.state.value}
                                            error={
                                                !field.state.meta.isValid && field.state.meta.errors.join(', ')
                                            }
                                            onBlur={field.handleBlur}
                                            min={0}
                                            max={100}
                                            onChange={(e) => {
                                                if (e) {
                                                    field.handleChange(e as number);
                                                }
                                            }}
                                        />
                                    </>
                                }}
                            />
                        </>
                    }}
                />
            </Stack>

            <form.Subscribe
                selector={(state) => [state.canSubmit, state.isSubmitting]}
                children={([canSubmit, isSubmitting]) => (
                    <Group
                        justify="flex-end"
                        gap="sm"
                    >
                        <Button
                            data-cy="create"
                            mt="sm"
                            type="submit"
                            disabled={!canSubmit}
                            loading={isSubmitting}
                            onClick={() => {
                                form.handleSubmit()
                            }}
                        >
                            Create
                        </Button>
                    </Group>
                )}
            />
        </form>
    </>
}
