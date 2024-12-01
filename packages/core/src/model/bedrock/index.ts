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
        private readonly _responseSubject = new Subject<ModelResponseEvent>(),
        private readonly _bedrockClient: BedrockRuntimeClient = new BedrockRuntimeClient({region: _config.settings.region})
    ) {
        super();
    }

    private async _send(
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
        if (responseMessageContents.length > 0) {
            this._responseSubject.next({
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
                const toolResults = await Promise.all(toolUseBlocks.map(async (toolUseBlock) => {
                    return {
                        toolResult: await mapBedrockToolUseBlockToToolUseResult(
                            toolUseBlock,
                            async (args) => await this._toolPluginManager.invokeTool(toolUseBlock.name!, args)
                        )
                    } as ContentBlock.ToolResultMember;
                }));

                this._send({
                    ...command, messages: [...command.messages, response.output?.message!, {
                        role: 'user',
                        content: toolResults
                    }]
                }, maxRecursion--);
                break;
            case 'max_tokens':
                this._responseSubject.next(mapBedrockResponseToMaxTokensEvent(response));
                break;
            default:
                this._responseSubject.next(mapBedrockResponseToEndEvent(response));
        }
    }

    responses(): Subject<ModelResponseEvent> {
        return this._responseSubject;
    }

    send(prompt: Prompt) {
        const toolConfig = mapPluginManagerToBedrockToolConfiguration(this._toolPluginManager.getAvailableTools());

        (async () => {
            const messages = await this._conversationStore.getHistory();
            await this._send(
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
    }
}