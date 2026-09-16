import { Alert } from "@mantine/core"
import type { ReactElement } from "react"

export function LoadingError(): ReactElement {
    return <Alert
        mt="sm"
        variant='light'
        color='red'
        title='Unknown loading error'
        data-cy="error"
    >
        There was an unknown error while loading the data. Please try again later.
    </Alert>
}

export function LoadingError404(): ReactElement {
    return <Alert
        mt="sm"
        variant='light'
        color='red'
        title='Appraisal not found'
        data-cy="error"
    >
        A appraisal with that code could not be found.
    </Alert>
}
