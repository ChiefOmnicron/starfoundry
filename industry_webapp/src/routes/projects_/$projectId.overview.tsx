import { Stack, Table, Title } from '@mantine/core';
import { MarkdownView } from '@starfoundry/components/detailView/MarkdownView';
import { ItemList } from '@starfoundry/components/list/ItemList';
import { CopyText } from '@starfoundry/components/misc/CopyText';
import { LoadingAnimation } from '@starfoundry/components/misc/LoadingAnimation';
import { LoadingError } from '@starfoundry/components/misc/LoadingError';
import { useFetchProject } from '@starfoundry/components/services/projects/fetch';
import { useFetchProjectCost } from '@starfoundry/components/services/projects/fetchCost';
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

    const {
        isPending: isPendingCost,
        isError: isErrorCost,
        data: projectCost,
    } = useFetchProjectCost(projectId);

    if (isPending || isPendingCost) {
        return LoadingAnimation();
    }

    if (isError || isErrorCost) {
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
                                value={projectCost.sell_price}
                                number
                                withUnit
                            />
                        </Table.Td>
                    </Table.Tr>
                </Table.Tbody>
            </Table>

            <Title order={2}>Finances</Title>
            <Table variant="vertical" layout="fixed" withTableBorder>
                <Table.Tbody>
                    <Table.Tr>
                        <Table.Th w={240}>Job cost</Table.Th>
                        <Table.Td>
                            <CopyText
                                value={projectCost.job_cost}
                                number
                                withUnit
                            />
                        </Table.Td>
                    </Table.Tr>
                    <Table.Tr>
                        <Table.Th>Market cost</Table.Th>
                        <Table.Td>
                            <CopyText
                                value={projectCost.market_cost}
                                number
                                withUnit
                            />
                        </Table.Td>
                    </Table.Tr>
                    <Table.Tr>
                        <Table.Th>Miscellaneous Cost</Table.Th>
                        <Table.Td>
                            <CopyText
                                value={projectCost.misc_cost}
                                number
                                withUnit
                            />
                        </Table.Td>
                    </Table.Tr>
                    <Table.Tr>
                        <Table.Th>Excess cost</Table.Th>
                        <Table.Td>
                            <CopyText
                                value={projectCost.excess_cost}
                                number
                                withUnit
                            />
                        </Table.Td>
                    </Table.Tr>
                    <Table.Tr>
                        <Table.Th>Stock cost</Table.Th>
                        <Table.Td>
                            <CopyText
                                value={projectCost.stock_cost}
                                number
                                withUnit
                            />
                        </Table.Td>
                    </Table.Tr>
                    <Table.Tr>
                        <Table.Th>Total cost</Table.Th>
                        <Table.Td>
                            <CopyText
                                value={projectCost.job_cost + projectCost.market_cost + projectCost.misc_cost - projectCost.excess_cost}
                                number
                                withUnit
                            />
                        </Table.Td>
                    </Table.Tr>
                    <Table.Tr>
                        <Table.Th>Sell Price</Table.Th>
                        <Table.Td>
                            <CopyText
                                value={projectCost.sell_price}
                                number
                                withUnit
                            />
                        </Table.Td>
                    </Table.Tr>
                    <Table.Tr>
                        <Table.Th>Profit</Table.Th>
                        <Table.Td>
                            <CopyText
                                value={projectCost.sell_price - projectCost.job_cost - projectCost.market_cost - projectCost.misc_cost + projectCost.excess_cost}
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
