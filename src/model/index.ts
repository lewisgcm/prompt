import {ModelConfig} from "../config";
import {ToolPluginManager} from "../tool";
import {Model} from "./model";
import {BedrockConfig, BedrockModel} from "./bedrock";

export function resolveModel(name: string, config: ModelConfig, toolPluginManager: ToolPluginManager): undefined | Model {
    switch (config.provider) {
        case 'bedrock':
            return new BedrockModel(toolPluginManager, config as BedrockConfig);
    }
    return undefined;
}