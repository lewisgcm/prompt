import {describe, expect, jest, test} from "@jest/globals";
import {
    BedrockRuntimeClient,
    ConverseCommandInput,
    ConverseCommandOutput
} from "@aws-sdk/client-bedrock-runtime";

import {BedrockModel} from "./index";
import {ToolPluginManager} from "../../tool";
import {BedrockStore} from "./store";
import {firstValueFrom, lastValueFrom, ReplaySubject, Subject, tap} from "rxjs";
import {ModelResponseEvent} from "../model";

describe('bedrock model tests', () => {
    test('simple send test', async () => {
        const store = jest.mocked(new BedrockStore("path", "conversation-id"));
        const bedrockClient = jest.mocked(new BedrockRuntimeClient());

        store.getHistory = jest.fn(() => Promise.resolve([]));
        // @ts-ignore
        bedrockClient.send = jest.fn((input: ConverseCommandInput) => Promise.resolve({
            stopReason: 'end_turn',
            output: {
                message: {
                    role: 'assistant',
                    content: [
                        {
                            text: 'test'
                        }
                    ]
                }
            },
            usage: {
                totalTokens: 3,
                outputTokens: 2,
                inputTokens: 1
            }
        } as ConverseCommandOutput));

        const responseSubject = new Subject<ModelResponseEvent>();
        const plugin = await ToolPluginManager.fromEntryFiles({});
        const bedrockModel = new BedrockModel(
            plugin,
            {provider: 'bedrock', settings: {region: 'us-east-1', "model-id": 'id'}},
            store,
            responseSubject,
            bedrockClient
        );

        bedrockModel.send({type: 'text', value: 'text'});

        let messages: ModelResponseEvent[] = [];
        await lastValueFrom(responseSubject.pipe(tap((message) => {
            messages.push(message);
            if (message.event == 'end') {
                responseSubject.complete();
            }
        })));

        expect(store.getHistory.mock.calls.length).toBe(1);
        expect(store.messages).toStrictEqual([
            {role: 'user', content: [{text: 'text'}]},
            {role: 'assistant', content: [{text: 'test'}]}
        ]);

        expect(bedrockClient.send.mock.calls.length).toBe(1);
        expect(messages).toStrictEqual([
            {
                event: 'response',
                content: [
                    {
                        type: 'text',
                        value: 'test'
                    }
                ]
            },
            {
                event: 'end',
                usage: {
                    totalTokens: 3,
                    outputTokens: 2,
                    inputTokens: 1,
                    latency: 0
                }
            }
        ] as ModelResponseEvent[]);
    });

    test('max tokens test', async () => {
        const store = jest.mocked(new BedrockStore("path", "conversation-id"));
        const bedrockClient = jest.mocked(new BedrockRuntimeClient());

        store.getHistory = jest.fn(() => Promise.resolve([]));
        // @ts-ignore
        bedrockClient.send = jest.fn((input: ConverseCommandInput) => Promise.resolve({
            stopReason: 'max_tokens',
            usage: {
                totalTokens: 3,
                outputTokens: 2,
                inputTokens: 1
            }
        } as ConverseCommandOutput));

        const responseSubject = new ReplaySubject<ModelResponseEvent>();
        const plugin = await ToolPluginManager.fromEntryFiles({});
        const bedrockModel = new BedrockModel(
            plugin,
            {provider: 'bedrock', settings: {region: 'us-east-1', "model-id": 'id'}},
            store,
            responseSubject,
            bedrockClient
        );

        bedrockModel.send({type: 'text', value: 'text'});

        let messages: ModelResponseEvent[] = [];
        await firstValueFrom(responseSubject.pipe(tap((message) => {
            messages.push(message);
        })));

        expect(store.getHistory.mock.calls.length).toBe(1);
        expect(store.messages).toStrictEqual([
            {role: 'user', content: [{text: 'text'}]},
        ]);

        expect(bedrockClient.send.mock.calls.length).toBe(1);
        expect(messages).toStrictEqual([
            {
                event: 'max_tokens',
                usage: {
                    totalTokens: 3,
                    outputTokens: 2,
                    inputTokens: 1,
                    latency: 0
                }
            }
        ] as ModelResponseEvent[]);
    });
});