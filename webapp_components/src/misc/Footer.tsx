import { AppShell, Group } from "@mantine/core";
import { Link } from "@tanstack/react-router";

export function Footer({
    legalRoute,
}: FooterProps) {
    return <AppShell.Footer
        style={{
            fontSize: '10px',
        }}
    >
        <Group justify='flex-end'>
            <div>
                All

                <Link
                    to={legalRoute}
                    style={{
                        color: 'var(--mantine-color-blue-9)',
                        padding: '5px',
                        textDecoration: 'none',
                    }}
                >
                    Eve related materials
                </Link>

                are property of

                <a
                    href="https://www.ccpgames.com"
                    target="_blank"
                    style={{
                        color: 'var(--mantine-color-blue-9)',
                        padding: '5px',
                        textDecoration: 'none',
                    }}
                >
                    Fenris Creations
                </a>

                See

                <Link
                    to={legalRoute}
                    style={{
                        color: 'var(--mantine-color-blue-9)',
                        paddingLeft: '5px',
                        textDecoration: 'none',
                    }}
                >
                    legal notice
                </Link>
                .
            </div>
        </Group>
    </AppShell.Footer>
}

export type FooterProps = {
    legalRoute: any,
}
