import type { Uuid } from "@/services/utils";
import type { ComponentType } from "react";

export function withStructure<T extends StructureProps>(
    WrappedComponent: ComponentType<T>,
): ComponentType<T> {
    return(
        props: T,
    ) => {
        console.log(props)

        return (
            <div>
                <h2>Parent Component</h2>
                <WrappedComponent
                    {...(props as T)}
                    structure={"a"}
                />
            </div>
        );
    };
}

export type StructureProps = {
    structureId: Uuid,
}

export function withTest123(
    WrappedComponent: any,
): any {
    return(
        componentProps: {
            structureId: string
        },
    ) => (
        props: any,
    ) => {
        console.log(props)

        return (
            <div>
                <h2>Parent Component</h2>
                <WrappedComponent
                    {...props}
                    structure={"a"}
                />
            </div>
        );
    };
}
