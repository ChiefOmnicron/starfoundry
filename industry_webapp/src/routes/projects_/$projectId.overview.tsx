import { Stack, Table, Title } from '@mantine/core';
import { MarkdownView } from '@starfoundry/components/detailView/MarkdownView';
import { ItemList } from '@starfoundry/components/list/ItemList';
import { CopyText } from '@starfoundry/components/misc/CopyText';
import { LoadingAnimation } from '@starfoundry/components/misc/LoadingAnimation';
import { LoadingError } from '@starfoundry/components/misc/LoadingError';
import { useFetchProject } from '@starfoundry/components/services/projects/fetch';
import { createFileRoute } from '@tanstack/react-router';

export const Route = createFileRoute('/projects_/$projectId/overview')({
    component: RouteComponent,
});

function RouteComponent() {
    const { projectId } = Route.useParams();

    const {
        isPending,
        isError,
        data: project,
    } = useFetchProject(projectId);

    if (isPending) {
        return LoadingAnimation();
    }

    if (isError) {
        return LoadingError();
    }

    const showNotes = () => {
        if (project.note) {
            return <>
                <Title order={2}>Notes</Title>
                <MarkdownView
                    content={project.note}
                />
            </>
        } else {
            return <></>;
        }
    }

    return <>
        <Stack>
            <Title order={2}>General Information</Title>
            <Table variant="vertical" layout="fixed" withTableBorder>
                <Table.Tbody>
                    <Table.Tr>
                        <Table.Th w={240}>Orderer</Table.Th>
                        <Table.Td>
                            <CopyText
                                value={project.orderer}
                            />
                        </Table.Td>
                    </Table.Tr>
                    <Table.Tr>
                        <Table.Th>Sell Price</Table.Th>
                        <Table.Td>
                            <CopyText
                                value={project.sell_price}
                                number
                                withUnit
                            />
                        </Table.Td>
                    </Table.Tr>
                </Table.Tbody>
            </Table>

            {showNotes()}

            <Title order={2}>Products</Title>
            <ItemList
                selected={project.products.map(x => x.item)}
            />
        </Stack>
    </>
}
