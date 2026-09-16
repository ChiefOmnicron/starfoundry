import { Stack } from '@mantine/core';
import { AppraisalInput } from '@/components/Input';
import { createAppraisal, type CreateAppraisal } from '@starfoundry/components/services/appraisal/create';
import { createFileRoute, useNavigate } from '@tanstack/react-router'
import { Route as FetchAppraisalRoute } from '@/routes/appraisals_/$code.index';
import { useMutation } from '@tanstack/react-query';
import { useState } from 'react';

export const Route = createFileRoute('/appraisals/')({
    component: RouteComponent,
})

function RouteComponent() {
    const navigation = useNavigate();

    const [createError, setCreateError] = useState<boolean>(false);

    const createAppraisalMutation = useMutation({
        mutationFn: async (info: CreateAppraisal) => {
            return await createAppraisal(info);
        },
        onError: () => {
            setCreateError(true);
        },
        onSuccess: (data) => {
            navigation({
                to: FetchAppraisalRoute.to,
                params: {
                    code: data.code,
                },
            })
        },
    });

    return <>
        <Stack>
            <AppraisalInput
                showCreateError={createError}
                onCreate={(info: CreateAppraisal) => createAppraisalMutation.mutate(info)}
            />
        </Stack>
    </>
}
