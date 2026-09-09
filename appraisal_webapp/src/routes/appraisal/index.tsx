import { AppraisalInput } from '@/services/appraisal/-components/Input';
import { AppraisalItemTable } from '@/services/appraisal/-components/Table';
import { createAppraisal, type Appraisal, type CreateAppraisal } from '@/services/appraisal/create';
import { Grid } from '@mantine/core';
import { useMutation } from '@tanstack/react-query';
import { createFileRoute } from '@tanstack/react-router'
import { useState } from 'react';

export const Route = createFileRoute('/appraisal/')({
    component: RouteComponent,
})

function RouteComponent() {
    const [appraisal, setAppraisal] = useState<Appraisal | undefined>(undefined);

    const createAppraisalMutation = useMutation({
        mutationFn: async (info: CreateAppraisal) => {
            return await createAppraisal(info);
        },
        onError: () => {
        },
        onSuccess: (data) => {
            setAppraisal(data)
        },
    });

    if (!appraisal) {
        return <>
            <AppraisalInput
                onCreate={(info: CreateAppraisal) => createAppraisalMutation.mutate(info)}
            />
        </>
    }

    return <>
        <Grid>
            <Grid.Col span={{ base: 12, sm: 4}}>
                <AppraisalInput
                    fullSize

                    onCreate={(info: CreateAppraisal) => createAppraisalMutation.mutate(info)}
                />
            </Grid.Col>

            <Grid.Col span={{ base: 12, sm: 8}}>
                <AppraisalItemTable
                    appraisal={appraisal}
                />
            </Grid.Col>
        </Grid>
    </>
}
