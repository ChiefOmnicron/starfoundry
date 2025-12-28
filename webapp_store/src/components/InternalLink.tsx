import { Link } from "@tanstack/react-router";

export function InternalLink({
    to,
    params,
    content,

    target = '_self',
}: Props) {
    return <Link
        to={ to }
        params={ params }
        target={ target }
        style={{
            color: 'var(--mantine-color-blue-4)',
            fontSize: 'var(--mantine-font-size-sm)',
            textDecoration: 'none'
        }}
    >
        { content }
    </Link>
}

export type Props = {
    // tanstack uses some insane typing, so just use any
    to: any,
    // tanstack uses some insane typing, so just use any
    params?: any,
    target?: '_blank' | '_self',

    content: string,
};
