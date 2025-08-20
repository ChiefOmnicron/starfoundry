import { Center, Loader } from "@mantine/core";
import type { ReactElement } from "react";

export default function LoadingAnimation(): ReactElement {
    return <Center data-cy="loading">
        <Loader type='bars' />
    </Center>
}
