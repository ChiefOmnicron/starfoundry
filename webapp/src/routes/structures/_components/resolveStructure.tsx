import { resolveStructure, type ResolveStructureResponse } from "@/services/structure/resolveStructure";
import { Button, TextInput } from "@mantine/core";
import { useMutation } from "@tanstack/react-query";
import { useState } from "react";

export function ResolveStructure(
    {
        onSuccess,
        onError,
    }: Props,
) {
    const [structureInput, setStructureInput] = useState<string>('');

    const structure = useMutation({
        mutationFn: async (structureId: number) => {
            return await resolveStructure(structureId);
        },
        onError: (error) => {
            onError(error);
        },
        onSuccess: (data) => {
            onSuccess(data);
        },
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
            data-cy="name"
            label="Eve-Structure ID"
            description="Either an EVE-Structure ID or a chat link to a structure"
            placeholder="Structure ID or '[00:00:00] CharacterName > <url=showinfo:35834//1000000000000>StructureSystem - StructureName</url>'"
            name="Structure ID"
            onChange={(e) => setStructureInput(e.currentTarget.value)}
            rightSectionWidth="12.5%"
            rightSection={
                <Button
                    onClick={() => resolve()}
                    style={{
                        width: '100%'
                    }}
                >
                    Resolve structure
                </Button>
            }
        />
    </>
}

export type Props = {
    onSuccess: (structure: ResolveStructureResponse) => void;
    onError:   (error: Error) => void,
}
