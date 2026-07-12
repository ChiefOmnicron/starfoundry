import { Alert, Button, Group } from "@mantine/core";
import { BadgeWrapper } from "@internal/wrapper/Badge";
import { createColumnHelper, getCoreRowModel, useReactTable } from "@tanstack/react-table";
import { deleteTag } from "@internal/services/tags/delete";
import { LIST_TAGS, type Tag } from "@internal/services/tags/list";
import { ModalWrapper } from "@internal/wrapper/Modal";
import { TableWrapper } from "@internal/wrapper/Table";
import { UpdateTag } from "./Update";
import { useDisclosure } from "@mantine/hooks";
import { useMutation, type MutationFunctionContext } from "@tanstack/react-query";
import { useState, type ReactElement } from "react";
import type { Uuid } from "@internal/services/utils";

export function TagList({
    tags,
}: TagListProps): ReactElement {
    const [opened, { open, close }] = useDisclosure(false);
    const [selectedTag, setSelectedTag] = useState<Tag | undefined>(undefined);

    const [hasError, setHasError] = useState<boolean>(false);
    const [isSuccess, setIsSuccess] = useState<boolean>(false);

    const deleteTagMutation = useMutation({
        mutationFn: (id: Uuid) => deleteTag(id),
        onError: () => {
            setIsSuccess(false);
            setHasError(true);
        },
        onSuccess: (_data, _variables, _onMutateResult, context: MutationFunctionContext) => {
            context.client.invalidateQueries({ queryKey: [LIST_TAGS] });
            setIsSuccess(true);
            setHasError(false);
        },
    });

    const updateTag = () => {
        return <ModalWrapper
            opened={ opened }
            close={ close }
            title="Update tag"
        >
            {
                selectedTag
                ?   <UpdateTag
                        tag={selectedTag}
                        onUpdate={() => close()}
                    />
                :   <></>
            }
        </ModalWrapper>
    }

    const columnHelper = createColumnHelper<Tag>();
    const columns = [
        columnHelper.display({
            id: 'tag',
            cell: props => <BadgeWrapper
                color={props.row.original.color}
            >
                {props.row.original.content}
            </BadgeWrapper>,
            header: () => 'Tag',
            size: 10,
        }),
        columnHelper.display({
            id: 'typ',
            cell: props => props.row.original.typ,
            header: () => 'Type',
            size: 5
        }),
        columnHelper.display({
            id: 'action',
            cell: props => <Group
                    justify="flex-end"
                >
                <Button
                    variant="transparent"
                    onClick={() => {
                        setSelectedTag(props.row.original);
                        open()
                    }}
                >
                    Update
                </Button>

                <Button
                    color="red.9"
                    variant="transparent"
                    onClick={() => deleteTagMutation.mutate(props.row.original.id)}
                >
                    Delete
                </Button>
            </Group>,
            header: () => '',
            size: 25,
        }),
    ];

    const table = useReactTable<Tag>({
        columns: columns,
        data: tags,
        autoResetPageIndex: false,
        getCoreRowModel: getCoreRowModel(),
        getRowId: row => row.id,
    });

    const notification = () => {
        if (hasError) {
            return <Alert
                mt="sm"
                variant='light'
                color='red'
                title='Error while deleting'
                data-cy="successfulUpdate"
                onClose={ () => setHasError(false) }
                withCloseButton
            >
                Error while deleting
            </Alert>;
        }
        if (isSuccess) {
            return <Alert
                mt="sm"
                variant='light'
                color='green'
                title='Delete Success'
                data-cy="successfulUpdate"
                onClose={ () => setIsSuccess(false) }
                withCloseButton
            >
                Deleting was successful
            </Alert>;
        }
    }

    return <>
        {notification()}
        {updateTag()}

        <TableWrapper
            table={table}
        />
    </>;
}

export type TagListProps = {
    tags: Tag[];
}
