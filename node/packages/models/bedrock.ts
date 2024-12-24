// Import the fetch specific variant
import {fromSSO} from "@aws-sdk/credential-providers/dist-es/fromSSO";
import {
    BedrockRuntimeClient,
    ContentBlock,
    ConverseCommand,
    DocumentFormat,
    ImageFormat
} from "@aws-sdk/client-bedrock-runtime";
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
        requestTimeout: 30000,
    }),
    streamCollector: streamCollector
}

function mapPromptToBedrockContentBlock(prompt: Prompt): ContentBlock {
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

class BedrockModelPlugin implements ModelPlugin {
    runtimeClient: BedrockRuntimeClient;
    modelId: string;

    async prompt(prompt: Prompt): Promise<PromptResponse[]> {
        const input = mapPromptToBedrockContentBlock(prompt);
        const command = new ConverseCommand({
            modelId: "anthropic.claude-3-haiku-20240307-v1:0",
            messages: [
                {
                    role: "user",
                    content: [
                        input
                    ]
                }
            ]
        });
        const response = await this.runtimeClient.send(command);
        return response.output.message.content
            .map((r) => {
                if (r.text) {
                    return {type: 'text', value: r.text}
                } else if (r.image) {
                    return {type: 'image', format: r.image.format, value: r.image.source.bytes};
                } else if (r.document) {
                    return {type: 'document', format: r.document.format, value: r.document.source.bytes};
                } else {
                    return null;
                }
            })
            .filter((r) => !!r) as PromptResponse[];
    }

    async configure(configuration: Configuration) {
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
        console.log(JSON.stringify(await credentialsProvider()));
        // const runtimeClient = new BedrockRuntimeClient({
        //     ...defaultClientConfig,
        //     credentials: credentialsProvider
        // });
        //
        // const command = new ConverseCommand({
        //     modelId: "anthropic.claude-3-haiku-20240307-v1:0",
        //     messages: [
        //         {
        //             role: "user",
        //             content: [
        //                 {
        //                     text: "Hello!",
        //                 }
        //             ]
        //         }
        //     ]
        // });
        // const response = await runtimeClient.send(command);
        // console.log(JSON.stringify(response.output));
    }
}

export const plugin = new BedrockModelPlugin();