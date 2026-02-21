import { Image } from "@mantine/core";
import Markdown from "react-markdown";
import type { ReactElement } from "react";

export function MarkdownView({
    content,
}: MarkdownViewProps): ReactElement {
    return <>
        <Markdown
            components={{
                img(x: any) {
                    return <Image
                        src={x.src}
                    />
                }
            }}
        >
            {content}
        </Markdown>
    </>
}

export type MarkdownViewProps = {
    content: string;
}
