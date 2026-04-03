import { Grid, Input, InputWrapper } from "@mantine/core";
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
    disabled = false,

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
                        disabled={disabled}
                    />
                </Input.Wrapper>
            </Grid.Col>

            <Grid.Col span={6}>
                <InputWrapper
                    label='Preview'
                >
                    <MarkdownView content={content} />
                </InputWrapper>
            </Grid.Col>
        </Grid>
    </>
}

export type MarkdownEditorProps = {
    content: string;
    title: string;

    description?: string;
    height?: string;
    disabled?: boolean;

    onChange: (change: string) => void;
}
