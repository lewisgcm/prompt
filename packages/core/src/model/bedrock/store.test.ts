import {expect, test} from '@jest/globals';
import {Message} from "@aws-sdk/client-bedrock-runtime";

import * as fs from "node:fs";
import {BedrockStore} from "./store";

const SAVED_HISTORY = [
    {
        role: 'user',
        content: []
    }
] as Message[];

test('simple bedrock store test', async () => {
    const dir = "test_output/simple/.prompt";
    fs.rmSync("test_output/simple/.prompt/test-id", {force: true});

    const bedrockStore = new BedrockStore(dir, 'test-id');

    // Verify history was saved on existing object
    expect(await bedrockStore.getHistory()).toStrictEqual([]);
    bedrockStore.setHistory(SAVED_HISTORY);
    expect(await bedrockStore.getHistory()).toStrictEqual(SAVED_HISTORY);

    // Save the history
    await bedrockStore.flush();

    // Ready again
    const bedrockStoreAfterWrite = new BedrockStore(dir, 'test-id');
    expect(await bedrockStoreAfterWrite.getHistory()).toStrictEqual(SAVED_HISTORY);
});