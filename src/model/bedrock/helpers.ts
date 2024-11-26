import {
    ContentBlock,
    ConverseCommandOutput,
    DocumentFormat,
    ImageFormat,
    Tool,
    ToolConfiguration, ToolResultBlock, ToolUseBlock
} from "@aws-sdk/client-bedrock-runtime";

import {ModelResponseEvent, ModelResponseMessageContent, Prompt} from "../model";
import {ToolSchema} from "../../tool";

export function mapBedrockMessageBlockContentToModelResponseMessageContent(contentBlock: ContentBlock[]): ModelResponseMessageContent[] {
    return contentBlock.map((content) => {
        if (content.text) {
            return {
                type: 'text',
                value: content.text,
            } as ModelResponseMessageContent;
        }
        return null;
    }).filter((m) => m != null)
}

export function mapBedrockResponseToMaxTokensEvent(commandOutput: ConverseCommandOutput): ModelResponseEvent {
    return {
        event: 'max_tokens',
        usage: {
            totalTokens: commandOutput.usage?.totalTokens || 0,
            inputTokens: commandOutput.usage?.inputTokens || 0,
            outputTokens: commandOutput.usage?.outputTokens || 0,
            latency: commandOutput.metrics?.latencyMs || 0
        }
    };
}

export function mapBedrockResponseToEndEvent(commandOutput: ConverseCommandOutput): ModelResponseEvent {
    return {
        event: 'end',
        usage: {
            totalTokens: commandOutput.usage?.totalTokens || 0,
            inputTokens: commandOutput.usage?.inputTokens || 0,
            outputTokens: commandOutput.usage?.outputTokens || 0,
            latency: commandOutput.metrics?.latencyMs || 0
        }
    }
}

export function mapBedrockToolUseBlockToToolUseResult(toolUseBlock: ToolUseBlock, toolCall: (args: any) => any): ToolResultBlock {
    try {
        const response = toolCall(toolUseBlock.input);
        return {
            toolUseId: toolUseBlock.toolUseId,
            content: [
                {
                    json: response
                }
            ],
            status: 'success',
        } as ToolResultBlock;
    } catch (e) {
        return {
            toolUseId: toolUseBlock.toolUseId,
            content: [
                {
                    text: e,
                }
            ],
            status: 'error',
        } as ToolResultBlock;
    }
}

export function mapPromptToBedrockContentBlock(prompt: Prompt): ContentBlock {
    switch (prompt.type) {
        case 'text':
            return {text: prompt.value};
        case 'document':
            return {
                document: {
                    format: prompt.format as DocumentFormat,
                    source: {bytes: prompt.value},
                    name: prompt.name
                }
            };
        case 'image':
            return {image: {format: prompt.format as ImageFormat, source: {bytes: prompt.value}}}
    }
}

export function mapPluginManagerToBedrockToolConfiguration(config: {
    [key: string]: { schema: ToolSchema; }
}): ToolConfiguration | undefined {
    const tools = Object.entries(config).map(([key, value]) => {
        const requiredArguments = Object.entries(value.schema.arguments || {})
            .filter(([key, value]) => value.required)
            .map(([key, value]) => key);

        const properties = Object.entries(value.schema.arguments || {}).reduce((dict, [key, value]) => {
            return {
                ...dict,
                [key]: {
                    type: value.type,
                    description: value.description,
                },
            };
        }, {});

        return {
            toolSpec: {
                name: key,
                description: value.schema.description,
                inputSchema: {
                    json: {
                        type: 'object',
                        properties: properties,
                        required: requiredArguments
                    }
                }
            }
        } as Tool;
    });

    if (tools.length == 0) return undefined;

    return {
        tools: tools,
    };
}