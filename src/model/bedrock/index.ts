import {
    BedrockRuntimeClient,
    ContentBlock,
    ConverseCommand,
    Message,
    ToolConfiguration,
} from "@aws-sdk/client-bedrock-runtime";
import {Subject} from "rxjs";

import {Model, ModelResponseEvent, Prompt} from "../model";
import {ToolPluginManager} from "../../tool";
import {ModelConfig} from "../../config";
import {
    mapPromptToBedrockContentBlock,
    mapPluginManagerToBedrockToolConfiguration,
    mapBedrockToolUseBlockToToolUseResult,
    mapBedrockResponseToEndEvent,
    mapBedrockResponseToMaxTokensEvent,
    mapBedrockMessageBlockContentToModelResponseMessageContent
} from "./helpers";
import {BedrockStore} from "./store";

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
        private readonly _conversationStore: BedrockStore,
        private readonly _bedrockClient: BedrockRuntimeClient = new BedrockRuntimeClient({region: _config.settings.region})
    ) {
        super();
    }

    private async _send(
        subject: Subject<ModelResponseEvent>,
        command: {
            messages: Message[],
            modelId: string,
            toolConfig?: ToolConfiguration
        }, maxRecursion: number): Promise<void> {
        if (maxRecursion <= 0) {
            throw new Error(`Exceeded maximum recursion for agent interaction: ${MAX_RECURSION}`);
        }

        const response = await this._bedrockClient.send(new ConverseCommand(command));
        const responseMessageContents = mapBedrockMessageBlockContentToModelResponseMessageContent(response.output?.message?.content || []);
        if (responseMessageContents) {
            subject.next({
                event: "response",
                content: responseMessageContents,
            });
        }

        this._conversationStore.setHistory(
            [
                ...command.messages,
                ...(response.output?.message ? [response.output?.message] : [])
            ]
        );

        switch (response.stopReason) {
            case 'tool_use':
                const toolUseBlocks = response.output?.message?.content?.filter((c) => !!c.toolUse).map((c) => c.toolUse) || [];
                const toolResults = toolUseBlocks.map((toolUseBlock) => {
                    return {
                        toolResult: mapBedrockToolUseBlockToToolUseResult(
                            toolUseBlock,
                            (args) => this._toolPluginManager.invokeTool(toolUseBlock.name!, args)
                        )
                    } as ContentBlock.ToolResultMember;
                });

                this._send(subject, {
                    ...command, messages: [...command.messages, response.output?.message!, {
                        role: 'user',
                        content: toolResults
                    }]
                }, maxRecursion--);
                break;
            case 'max_tokens':
                subject.next(mapBedrockResponseToMaxTokensEvent(response));
                break;
            default:
                subject.next(mapBedrockResponseToEndEvent(response));
                subject.complete();
        }
    }

    send(prompt: Prompt): Subject<ModelResponseEvent> {
        const subject = new Subject<ModelResponseEvent>();
        const toolConfig = mapPluginManagerToBedrockToolConfiguration(this._toolPluginManager.getAvailableTools());

        (async () => {
            const messages = await this._conversationStore.getHistory();
            await this._send(
                subject,
                {
                    toolConfig: toolConfig,
                    modelId: this._config.settings["model-id"],
                    messages: [
                        ...messages,
                        {
                            role: 'user',
                            content: [
                                mapPromptToBedrockContentBlock(prompt)
                            ]
                        }
                    ],
                }, MAX_RECURSION);
        })();

        return subject;
    }
}