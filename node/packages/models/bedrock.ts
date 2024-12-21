// Import the fetch specific variant
import {fromSSO} from "@aws-sdk/credential-providers/dist-es/fromSSO";
import {BedrockRuntimeClient, ConverseCommand} from "@aws-sdk/client-bedrock-runtime";
import {BedrockClient, ListFoundationModelsCommand} from "@aws-sdk/client-bedrock";
import {FetchHttpHandler, streamCollector} from "@smithy/fetch-http-handler";

import {Configuration, ConfigurationStep, ModelPlugin, Prompt, PromptResponse} from '@prompt/types';

const defaultCredentialConfig = {
    filepath: "~/.aws/credentials",
    clientConfig: {
        region: 'us-east-1',
        requestHandler: new FetchHttpHandler({
            requestTimeout: 1000,
        }),
        streamCollector: streamCollector
    },
};

const defaultClientConfig = {
    region: 'us-east-1',
    requestHandler: new FetchHttpHandler({
        requestTimeout: 1000,
    }),
    streamCollector: streamCollector
}

class BedrockModelPlugin implements ModelPlugin {
    runtimeClient: BedrockRuntimeClient;
    modelId: string;

    async prompt(prompt: Prompt): Promise<PromptResponse[]> {
        return [{
            type: 'text',
            value: 'Hello from the model!'
        }];
    }

    configure(configuration: Configuration): void {
        const credentialsProvider = fromSSO({
            ...defaultCredentialConfig,
            clientConfig: {
                ...defaultCredentialConfig.clientConfig,
                region: configuration.region,
            }
        });

        this.modelId = configuration['model-id'] as string;
        this.runtimeClient = new BedrockRuntimeClient({
            ...defaultClientConfig,
            region: configuration.region as string,
            credentials: credentialsProvider
        });
    }

    configuration(): ConfigurationStep[] {
        return [
            {
                input: (context: Configuration) => ({
                    region: {
                        displayName: 'AWS Region',
                        type: 'select',
                        required: true,
                        options: ['us-east-1', 'us-east-2', 'eu-west-1']
                    }
                })
            },
            {
                input: async (context: Configuration) => {
                    const region = context.region as string;

                    if (!region) {
                        return {
                            'model-id': {
                                displayName: 'AWS Bedrock model',
                                type: 'select',
                                required: true,
                                options: ['Configure AWS region first']
                            }
                        };
                    }

                    const credentialsProvider = fromSSO({
                        ...defaultCredentialConfig,
                        clientConfig: {
                            ...defaultCredentialConfig.clientConfig,
                            region,
                        }
                    });
                    const client = new BedrockClient({
                        ...defaultClientConfig,
                        region,
                        credentials: credentialsProvider
                    });

                    const response = await client.send(new ListFoundationModelsCommand());
                    const modelChoices = response.modelSummaries
                        ?.filter((m) => m.inferenceTypesSupported?.includes('ON_DEMAND') && m.outputModalities?.includes('TEXT'))
                        ?.map((m) => m.modelId) || [];

                    return {
                        'model-id': {
                            displayName: 'AWS Bedrock model',
                            type: 'select',
                            required: true,
                            options: modelChoices
                        }
                    };
                }
            }
        ];
    }

    async test(): Promise<void> {
        const credentialsProvider = fromSSO(defaultCredentialConfig);
        const runtimeClient = new BedrockRuntimeClient({
            ...defaultClientConfig,
            credentials: credentialsProvider
        });

        const command = new ConverseCommand({
            modelId: "anthropic.claude-3-haiku-20240307-v1:0",
            messages: [
                {
                    role: "user",
                    content: [
                        {
                            text: "Hello!",
                        }
                    ]
                }
            ]
        });
        const response = await runtimeClient.send(command);
        console.log(JSON.stringify(response.output));
    }
}

export const plugin = new BedrockModelPlugin();