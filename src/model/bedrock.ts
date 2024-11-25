import {Model, Prompt} from "./model";
import {ToolPluginManager} from "../tool";
import {ModelConfig} from "../config";
import {
    BedrockRuntimeClient,
    ContentBlock,
    ConverseCommand, ImageFormat, Message,
    Tool,
    ToolConfiguration, DocumentFormat
} from "@aws-sdk/client-bedrock-runtime";
import {Subject} from "rxjs";

const MAX_RECURSION = 5;

export interface BedrockConfig extends ModelConfig {
    provider: 'bedrock',
    settings: {
        region?: string;
        'model-id': string;
    };
    tools?: string[];
}

export class BedrockModel extends Model {
    constructor(
        private readonly _toolPluginManager: ToolPluginManager,
        private readonly _config: BedrockConfig,
        private readonly _bedrockClient: BedrockRuntimeClient = new BedrockRuntimeClient({region: _config.settings.region})
    ) {
        super();
    }

    private async _send(
        subject: Subject<string>,
        command: {
            messages: Message[],
            modelId: string,
            toolConfig?: ToolConfiguration
        }, maxRecursion: number): Promise<void> {
        if (maxRecursion <= 0) {
            throw new Error(`Exceeded maximum recursion for agent interaction: ${MAX_RECURSION}`);
        }

        const response = await this._bedrockClient.send(new ConverseCommand(command));

        //console.dir(response, {depth: null});

        switch (response.stopReason) {
            case 'tool_use':
                pushMessageContentToSubscribers(subject, response.output?.message);
                const toolCalls = response.output?.message?.content?.filter((c) => !!c.toolUse).map((c) => c.toolUse) || [];
                const toolResults = toolCalls.map((c) => {
                    try {
                        const response = this._toolPluginManager.invokeTool(c.name!, c.input);
                        return {
                            toolResult: {
                                toolUseId: c.toolUseId,
                                content: [
                                    {
                                        json: response
                                    }
                                ],
                                status: 'success',
                            }
                        } as ContentBlock.ToolResultMember;
                    } catch (e) {
                        return {
                            toolResult: {
                                toolUseId: c.toolUseId,
                                content: [
                                    {
                                        text: e,
                                    }
                                ],
                                status: 'error',
                            }
                        } as ContentBlock.ToolResultMember;
                    }
                });

                this._send(subject, {
                    ...command, messages: [...command.messages, response.output?.message!, {
                        role: 'user',
                        content: toolResults
                    }]
                }, maxRecursion--);
                break;
            case 'max_tokens':
                throw new Error("max tokens for model has been reached");
            default:
                pushMessageContentToSubscribers(subject, response.output?.message);
                subject.complete();
        }
    }

    send(prompt: Prompt): Subject<string> {
        const subject = new Subject<string>();

        const toolConfig = buildBedrockToolConfig(this._toolPluginManager)
        this._send(
            subject,
            {
                toolConfig: toolConfig,
                modelId: this._config.settings["model-id"],
                messages: [
                    {
                        role: 'user',
                        content: [
                            buildBedrockContentFromPrompt(prompt)
                        ]
                    }
                ],
            }, MAX_RECURSION);

        return subject;
    }
}

function pushMessageContentToSubscribers(subject: Subject<string>, message?: Message) {
    message?.content?.forEach((content) => {
        if (content.text) {
            subject.next(content.text);
        }
    });
}

function buildBedrockContentFromPrompt(prompt: Prompt): ContentBlock {
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

function buildBedrockToolConfig(toolPluginManager: ToolPluginManager): ToolConfiguration | undefined {
    const config = toolPluginManager.getAvailableTools();
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