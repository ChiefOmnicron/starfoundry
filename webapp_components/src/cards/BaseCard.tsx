import { Card, Checkbox, Group, Loader } from "@mantine/core"
import type { ReactNode } from "react"

export function BaseCard({
    children,
    header,

    footer,
    bottom,

    loading = false,

    checkable = false,
    selected = false,
    onCheckChange = () => {},
}: BaseCardProps) {
    const selectEvent = (
        state: boolean,
    ) => {
        if (!checkable) {
            return;
        }

        if (state) {
            onCheckChange('checked');
        } else {
            onCheckChange('unchecked');
        }
    }

    const showCheckbox = () => {
        if (!checkable) {
            return <></>
        }

        return <>
            <Checkbox
                checked={selected}
                size='xs'
                onChange={(event) => {
                    selectEvent(event.currentTarget.checked);
                }}
            />
        </>
    }

    const showLoader = () => {
        if (!loading) {
            return <></>
        }

        return <>
            <Loader color="blue" size="xs" />
        </>
    }

    return <Card
        style={{
            padding: 0,
            border: selected ? '1px solid var(--mantine-color-blue-9)' : '',
        }}
        onClick={() => {
            if (!checkable) {
                return;
            }

            selectEvent(!selected);
        }}
    >
        <Card.Section
            style={{
                margin: '10px',
            }}
        >
            <Group
                justify="space-between"
            >
                {header}

                {showCheckbox()}
                {showLoader()}
            </Group>
        </Card.Section>

        <Card.Section
            style={{
                margin: '10px',
                height: '100%',
            }}
        >
            {children}
        </Card.Section>

        {footer}

        {bottom}
    </Card>
}

export type BaseCardProps = {
    children:       ReactNode,
    header:         ReactNode,

    footer?:        ReactNode,
    bottom?:        ReactNode,

    loading?:       boolean;

    checkable?:     boolean;
    selected?:      boolean;
    onCheckChange?: (state: 'checked' | 'unchecked') => void;
}
