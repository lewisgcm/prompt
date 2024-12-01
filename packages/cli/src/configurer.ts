import {BedrockClient, ListFoundationModelsCommand} from "@aws-sdk/client-bedrock";

import {select, checkbox, input} from "@inquirer/prompts";
import {Config} from "@prompt/core";
import {BedrockConfig} from "@prompt/core";

export async function addModel(config: Config): Promise<Config> {
    const provider = await select({
        message: 'Select an AI agent provider',
        choices: ['bedrock']
    });

    const modelName = await input({
        message: 'Enter a name for the model',
    });

    let modelConfig;
    switch (provider) {
        case 'bedrock':
            modelConfig = await addBedrockModel(config);
            break;
        default:
            throw new Error(`unknown model provider: ${provider}`);
    }

    config.models = config.models || {};
    config.models[modelName] = modelConfig;

    if (!config["default-model"]) {
        config["default-model"] = modelName;
    }

    return config;
}

async function addBedrockModel(config: Config): Promise<BedrockConfig> {
    const region: string = await select({
        message: 'Select AWS region',
        choices: ['us-east-1', 'us-east-2', 'us-west-2', 'eu-west-1', 'eu-central-2']
    });

    // CredentialsProviderError
    // ExitPromptError
    const client = new BedrockClient({region: region});
    const response = await client.send(new ListFoundationModelsCommand());
    const modelChoices = response.modelSummaries
        ?.filter((m) => m.inferenceTypesSupported?.includes('ON_DEMAND') && m.outputModalities?.includes('TEXT'))
        ?.map((m) => ({value: m.modelId, name: m.modelName})) || [];

    const modelId = await select({
        message: 'Select AWS region',
        choices: modelChoices
    });

    const toolOptions = Object.entries(config["tool-plugins"] || {}).map(([key, value]) => key);
    const selectedTools = toolOptions.length == 0 ? [] : await checkbox({
        message: 'Select model plugins',
        choices: toolOptions,
    });

    return {
        settings: {
            'model-id': modelId as string,
            region: region,
        },
        provider: 'bedrock',
        tools: selectedTools as string[]
    };
}