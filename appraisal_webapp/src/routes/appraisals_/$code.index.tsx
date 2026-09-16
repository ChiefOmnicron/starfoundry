import { Alert, Grid, Stack, Tabs } from "@mantine/core";
import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { AppraisalInput } from "../../components/Input";
import { GeneralTable } from "../../components/GeneralTable";
import { ItemTable } from "../../components/ItemTable";
import { useFetchAppraisal } from "@starfoundry/components/services/appraisal/fetch";
import { createAppraisal, type CreateAppraisal } from "@starfoundry/components/services/appraisal/create";
import { useMutation } from "@tanstack/react-query";
import { LoadingError, LoadingError404 } from "@starfoundry/components/misc/LoadingError";
import { LoadingAnimation } from "@starfoundry/components/misc/LoadingAnimation";
import { useState } from "react";

export const Route = createFileRoute('/appraisals_/$code/')({
    component: RouteComponent,
})

export function RouteComponent() {
    const navigation = useNavigate();
    const { code } = Route.useParams();

    const [createError, setCreateError] = useState<boolean>(false);

    const {
        isError,
        isFetching,
        data,
        error,
    } = useFetchAppraisal(code);

    const createAppraisalMutation = useMutation({
        mutationFn: async (info: CreateAppraisal) => {
            return await createAppraisal(info);
        },
        onError: () => {
            setCreateError(true);
        },
        onSuccess: (data) => {
            navigation({
                to: Route.to,
                params: {
                    code: data.code,
                },
            })
        },
    });

    const notification = () => {
        if (isFetching) {
            return LoadingAnimation();
        }

        if (data && data.invalid.length > 0) {
            return <Alert
                variant="outline"
                color="red"
                title="Invalid Items"
            >
                {data.invalid.map(x => x)}
            </Alert>
        }

        if (error && error.message === 'Error 404') {
            return LoadingError404();
        }

        if (isError || !data) {
            return LoadingError();
        }
    }

    return <>
        <Grid>
            <Grid.Col span={{ base: 12, sm: 4}}>
                <AppraisalInput
                    fullSize
                    showCreateError={createError}
                    content={data?.raw}

                    onCreate={(info: CreateAppraisal) => createAppraisalMutation.mutate(info)}
                />
            </Grid.Col>

            <Grid.Col span={{ base: 12, sm: 8}}>
                {notification()}

                <Tabs defaultValue="appraisal">
                    <Tabs.List>
                        <Tabs.Tab value="appraisal">
                            Appraisal
                        </Tabs.Tab>

                        <Tabs.Tab value="reprocessing">
                            Reprocessing
                        </Tabs.Tab>

                        <Tabs.Tab value="compression">
                            Compression
                        </Tabs.Tab>

                        <Tabs.Tab value="compare">
                            Compare
                        </Tabs.Tab>

                        <Tabs.Tab value="settings">
                            Settings
                        </Tabs.Tab>
                    </Tabs.List>

                    <Tabs.Panel value="appraisal">
                        {
                            data
                            ?   <>
                                    <Stack>
                                        <GeneralTable
                                            appraisal={data}
                                        />

                                        <ItemTable
                                            appraisal={data}
                                        />
                                    </Stack>
                                </>
                            :   <></>
                        }
                    </Tabs.Panel>
                </Tabs>
            </Grid.Col>
        </Grid>
    </>
}
