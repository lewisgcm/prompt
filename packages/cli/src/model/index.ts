import {ModelConfig} from "../config";
import {ToolPluginManager} from "../tool";
import {ConversationStore, Model} from "./model";
import {BedrockConfig, BedrockModel} from "./bedrock";
import {BedrockStore} from "./bedrock/store";

export function resolveConversationStore(config: ModelConfig, homeDir: string, conversationId: string): undefined | ConversationStore<any> {
    switch (config.provider) {
        case 'bedrock':
            return new BedrockStore(homeDir, conversationId);
    }
    return undefined;
}

export function resolveModel(config: ModelConfig, toolPluginManager: ToolPluginManager, conversationStore: ConversationStore<any>): undefined | Model {
    switch (config.provider) {
        case 'bedrock':
            return new BedrockModel(toolPluginManager, config as BedrockConfig, conversationStore as BedrockStore);
    }
    return undefined;
}