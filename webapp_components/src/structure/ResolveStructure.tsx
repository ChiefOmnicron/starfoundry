import { resolveStructure, type ResolveStructureResponse } from "@internal/services/structure/resolveStructure";
import { Button, Flex, TextInput } from "@mantine/core";
import { useMutation } from "@tanstack/react-query";
import { useState } from "react";

// Usage:
//
// <ResolveStructure
//   onError={(error) => {}}
//   onSuccess={(structure) => {}}
// >
//
export function ResolveStructure(
    {
        onSuccess,
        onError,
        onLoad = (_) => {},
    }: Props,
) {
    const [structureInput, setStructureInput] = useState<string>('');
    const [isLoading, setIsLoading] = useState<boolean>(false);

    const structure = useMutation({
        mutationFn: async (structureId: number) => {
            setIsLoading(true);
            onLoad(true);
            return await resolveStructure(structureId);
        },
        onError: (error) => {
            onError(error);
        },
        onSuccess: (data) => {
            onSuccess(data);
        },
        onSettled: (_) => {
            setIsLoading(false);
            onLoad(false);
        }
    });

    const resolve = () => {
        let structureId!: number;
        if (structureInput.indexOf('url=showinfo') >= 0) {
            let regex = /<url=showinfo:[0-9]*\/\/([0-9]*)>/;
            let match = structureInput.match(regex) || [];
            structureId = parseInt(match[1]);
        } else {
            structureId = parseInt(structureInput);
        }

        structure.mutate(structureId);
    }

    return <>
        <TextInput
            data-1p-ignore
            data-cy="structureId"
            label="Eve-Structure ID"
            description="Either an EVE-Structure ID or a chat link to a structure"
            placeholder="Structure ID or '[00:00:00] CharacterName > <url=showinfo:35834//1000000000000>StructureSystem - StructureName</url>'"
            name="Structure ID"
            onChange={(e) => setStructureInput(e.currentTarget.value)}
        />
        <Flex
            justify="flex-end"
            gap="sm"
        >
            <Button
                data-cy="resolveStructure"
                onClick={() => resolve()}
                disabled={isLoading}
                loading={isLoading}
            >
                Resolve structure
            </Button>
        </Flex>
    </>
}

export type Props = {
    onSuccess: (structure: ResolveStructureResponse) => void;
    onError:   (error: Error) => void,
    onLoad?:   (loading: boolean) => void,
}
