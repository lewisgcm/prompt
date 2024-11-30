import {describe, expect, test} from "@jest/globals";
import {ToolPluginManager} from "./tool";
import path from "path";

describe("tool tests", () => {
    test("empty plugin", async () => {
        const plugin = await ToolPluginManager.fromEntryFiles({});

        expect(plugin.getAvailableTools()).toStrictEqual({});
        expect(await plugin.invokeTool('tool doesnt exist', null)).toStrictEqual(undefined);
    });

    test("test plugin", async () => {
        const testPluginFile = path.join(__dirname, "..", "test", "plugin");
        const plugin = await ToolPluginManager.fromEntryFiles({
            test: testPluginFile,
        });

        expect(Object.keys(plugin.getAvailableTools())).toStrictEqual(['test']);
        expect(plugin.getAvailableTools()['test'].schema).toStrictEqual({
            description: 'This is a test plugin',
            arguments: {
                input: {
                    type: 'string',
                    description: 'Test argument',
                    required: true,
                }
            }
        });
        expect(await plugin.invokeTool('test', {input: 'input'})).toStrictEqual({
            testOutput: 'Some output input'
        });
        expect(await plugin.invokeTool('tool doesnt exist', null)).toStrictEqual(undefined);
    });
});