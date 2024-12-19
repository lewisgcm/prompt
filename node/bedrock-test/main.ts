// Import the fetch specific variant
import {fromSSO} from "@aws-sdk/credential-providers/dist-es/fromSSO";
import {BedrockRuntimeClient, ConverseCommand} from "@aws-sdk/client-bedrock-runtime";
import {BedrockClient} from "@aws-sdk/client-bedrock";
import {FetchHttpHandler, streamCollector} from "@smithy/fetch-http-handler";

const credentialsProvider = fromSSO({
    filepath: "~/.aws/credentials",
    clientConfig: {
        region: 'us-east-1',
        requestHandler: new FetchHttpHandler({
            requestTimeout: 30_000,
        }),
        streamCollector: streamCollector
    },
});

const runtimeClient = new BedrockRuntimeClient({
    credentials: credentialsProvider,
    region: 'us-east-1',
    requestHandler: new FetchHttpHandler({
        requestTimeout: 30_000,
    }),
    streamCollector: streamCollector
});

export const configuration = async () => {
    const client = new BedrockClient({
        
    });
}

export const test = async () => {
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
