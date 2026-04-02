import { Grid, Input } from "@mantine/core";
import { langs } from "@uiw/codemirror-extensions-langs";
import { MarkdownView } from "../detailView/MarkdownView";
import { vscodeDark } from "@uiw/codemirror-theme-vscode";
import ReactCodeMirror from "@uiw/react-codemirror";
import type { ReactElement } from "react";

export function MarkdownEditor({
    content,
    title,

    description = 'Markdown is supported',
    height = '400px',

    onChange,
}: MarkdownEditorProps): ReactElement {
    return <>
        <Grid>
            <Grid.Col span={6}>
                <Input.Wrapper
                    label={title}
                    description={description}
                >
                    <ReactCodeMirror
                        value={content}
                        height={height}
                        onChange={onChange}
                        extensions={[
                            langs.markdown(),
                        ]}
                        theme={vscodeDark}
                    />
                </Input.Wrapper>
            </Grid.Col>

            <Grid.Col span={6}>
                <MarkdownView content={content} />
            </Grid.Col>
        </Grid>
    </>
}

export type MarkdownEditorProps = {
    content: string;
    title: string;

    description?: string;
    height?: string;

    onChange: (change: string) => void;
}
