import {fromSSO} from "@aws-sdk/credential-providers/dist-es/fromSSO"; // Import the fetch specific variant
import {BedrockRuntimeClient, ConverseCommand} from "@aws-sdk/client-bedrock-runtime";
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
const client = new BedrockRuntimeClient({
    credentials: credentialsProvider,
    region: 'us-east-1',
    requestHandler: new FetchHttpHandler({
        requestTimeout: 30_000,
    }),
    streamCollector: streamCollector
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

try {
    console.log("here?")
    const re = await client.send(command);
    console.log(JSON.stringify(re));
} catch (e) {
    console.log("here2")
    console.log(e);
}

export const testy = async () => {
    return ""; //client.send(command);
}