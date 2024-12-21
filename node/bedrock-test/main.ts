// Import the fetch specific variant
import {fromSSO} from "@aws-sdk/credential-providers/dist-es/fromSSO";
import {BedrockRuntimeClient, ConverseCommand} from "@aws-sdk/client-bedrock-runtime";
import {BedrockClient, ListFoundationModelsCommand} from "@aws-sdk/client-bedrock";
import {FetchHttpHandler, streamCollector} from "@smithy/fetch-http-handler";

const defaultCredentialConfig = {
    filepath: "~/.aws/credentials",
    clientConfig: {
        region: 'us-east-1',
        requestHandler: new FetchHttpHandler({
            requestTimeout: 30_000,
        }),
        streamCollector: streamCollector
    },
};

const defaultClientConfig = {
    region: 'us-east-1',
    requestHandler: new FetchHttpHandler({
        requestTimeout: 30_000,
    }),
    streamCollector: streamCollector
}

interface ModelPlugin {
    configure(configuration: Configuration): void;

    configuration(): ConfigurationStep[];

    test(): Promise<void>
}

type ConfigurationType = string | number | boolean | null;

interface Configuration {
    [key: string]: ConfigurationType | ConfigurationType[]
}

interface ConfigurationInput {
    displayName: string;
    type: 'select' | 'bool' | 'integer' | 'float' | 'string';
    options?: string[];
    required: boolean;
}

interface ConfigurationStep {
    input: (context: Configuration) => Promise<{ [key: string]: ConfigurationInput }> | {
        [key: string]: ConfigurationInput
    };
}

class BedrockModelPlugin implements ModelPlugin {
    configure(configuration: Configuration): void {
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