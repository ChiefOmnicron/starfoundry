import { Grid } from "@mantine/core";
import { langs } from "@uiw/codemirror-extensions-langs";
import { MarkdownView } from "./MarkdownView";
import { vscodeDark } from "@uiw/codemirror-theme-vscode";
import ReactCodeMirror from "@uiw/react-codemirror";
import type { ReactElement } from "react";

export function MarkdownEditor({
    content,

    onChange,
}: MarkdownEditorProps): ReactElement {
    return <>
        <Grid>
            <Grid.Col span={6}>
                <ReactCodeMirror
                    value={content}
                    height="400px"
                    onChange={onChange}
                    extensions={[
                        langs.markdown(),
                    ]}
                    theme={vscodeDark}
                />
            </Grid.Col>
            <Grid.Col span={6}>
                <MarkdownView content={content} />
            </Grid.Col>
        </Grid>
    </>
}

export type MarkdownEditorProps = {
    content: string;

    onChange: (change: string) => void;
}
