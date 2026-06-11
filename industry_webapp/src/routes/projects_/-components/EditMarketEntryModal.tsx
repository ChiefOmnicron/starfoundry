import { Button, Group, InputBase, InputWrapper, NumberInput, Stack } from "@mantine/core";
import { EveIcon } from "@starfoundry/components/misc/EveIcon";
import { ModalWrapper } from "@starfoundry/components/wrapper/Modal";
import { useForm } from "@tanstack/react-form";
import type { ProjectMarketEntry } from "@starfoundry/components/services/projects/listMarket";
import type { ReactElement } from "react";

export function EditMarketEntryModal({
    entry,

    onSave,

    opened,
    close,
}: EditMarketEntryModalProps): ReactElement {
    const form = useForm({
        defaultValues: {
            buyPrice: entry.cost,
            quantity: entry.quantity,
            source: entry.source,
        },
        onSubmit: async ({ value }) => {
            onSave({
                ...entry,
                cost: value.buyPrice,
                quantity: value.quantity,
                source: value.source,
            });
        }
    });

    return <>
        <ModalWrapper
            opened={opened}
            close={() => {
                form.reset();
                close();
            }}
            title="Update entry"
            size="50%"
        >
            <form
                onSubmit={(e) => {
                    e.preventDefault();
                    e.stopPropagation();
                    form.handleSubmit();
                }}
            >
                <Stack>
                    <InputWrapper
                        label="Item"
                    >
                        <Group>
                            <EveIcon
                                id={entry.item.type_id}
                            />

                            {entry.item.name}
                        </Group>
                    </InputWrapper>

                    <form.Field
                        name="quantity"
                        children={(field) => {
                            return <>
                                <NumberInput
                                    data-1p-ignore
                                    thousandSeparator
                                    data-cy="quantity"
                                    label="Quantity"
                                    description="Number of items that need to be bought or were bought"
                                    placeholder="1,000,000,000"
                                    id={field.name}
                                    name={field.name}
                                    value={field.state.value}
                                    withAsterisk
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
                        name="buyPrice"
                        children={(field) => {
                            return <>
                                <NumberInput
                                    data-1p-ignore
                                    thousandSeparator
                                    data-cy="buyPrice"
                                    label="Buy Price"
                                    description="Total cost of the material"
                                    placeholder="1,000,000,000"
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
                        name="source"
                        children={(field) => {
                            return <>
                                <InputBase
                                    data-1p-ignore
                                    data-cy="source"
                                    label="Source"
                                    description="Where the materials were bought"
                                    placeholder="Jita 4-4"
                                    id={field.name}
                                    name={field.name}
                                    value={field.state.value}
                                    error={
                                        !field.state.meta.isValid && field.state.meta.errors.join(', ')
                                    }
                                    onBlur={field.handleBlur}
                                    onChange={(e) => field.handleChange(e.target.value)}
                                />
                            </>
                        }}
                    />

                    <form.Subscribe
                        selector={(state) => [state.canSubmit, state.isSubmitting]}
                        children={([canSubmit, isSubmitting]) => (
                            <Group
                                justify="flex-end"
                                gap="sm"
                            >
                                <Button
                                    mt="sm"
                                    variant="subtle"
                                    color="gray"
                                    onClick={close}
                                >
                                    Close
                                </Button>
                                <Button
                                    data-cy="create"
                                    mt="sm"
                                    type="submit"
                                    disabled={!canSubmit}
                                    loading={isSubmitting}
                                >
                                    Update
                                </Button>
                            </Group>
                        )}
                    />
                </Stack>
            </form>
        </ModalWrapper>
    </>
}

export type EditMarketEntryModalProps = {
    entry: ProjectMarketEntry,

    onSave: (value: ProjectMarketEntry) => void;

    opened: boolean;
    close: () => void;
}
