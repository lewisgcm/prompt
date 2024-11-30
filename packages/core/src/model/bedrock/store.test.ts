import {expect, test} from '@jest/globals';
import {Message} from "@aws-sdk/client-bedrock-runtime";

import * as fs from "node:fs";
import {BedrockStore} from "./store";
import path from "path";
import {after} from "node:test";

const SAVED_HISTORY = [
    {
        role: 'user',
        content: []
    }
] as Message[];

const TEST_OUTPUT = path.join(__dirname, "..", "..", "..", "test", "test_output");

test('simple bedrock store test', async () => {
    const dir = path.join(TEST_OUTPUT, "simple");
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

after(() => {
    const dir = path.join(TEST_OUTPUT, "simple");
    fs.rmSync(path.join(dir, "test-id"), {force: true});
    fs.rmdirSync(path.join(dir));
});