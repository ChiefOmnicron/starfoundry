import { Text, Title } from '@mantine/core'
import { createFileRoute, Outlet } from '@tanstack/react-router'

export const Route = createFileRoute('/structures')({
    component: StructureHeader,
})

function StructureHeader() {
    return <>
        <Title
            data-cy="header"
            order={1}
            style={{
                marginBottom: '15px'
            }}
        >
            Structures
        </Title>
        <Text
            size='md'
            pb="md"
        >
            List of all structures you have added or are shared with you
        </Text>

        <Outlet />
    </>
}
