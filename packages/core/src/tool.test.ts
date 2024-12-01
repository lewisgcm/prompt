import {afterEach, describe, expect, jest, test} from "@jest/globals";
import {InvalidPluginError, ToolPluginManager} from "./tool";
import path from "path";

const TEST_PLUGIN_FILE = path.join(__dirname, "..", "test", "plugin");

describe("tool tests", () => {
    afterEach(() => {
        jest.resetModules();
    });

    test("empty plugin", async () => {
        const plugin = await ToolPluginManager.fromEntryFiles({});

        expect(plugin.getAvailableTools()).toStrictEqual({});
        expect(await plugin.invokeTool('tool doesnt exist', null)).toStrictEqual(undefined);
    });

    test("test plugin", async () => {
        const plugin = await ToolPluginManager.fromEntryFiles({
            test: TEST_PLUGIN_FILE,
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

    test("test invalid plugin schema - null description", async () => {
        jest.mock(TEST_PLUGIN_FILE, () => {
            return {
                schema: {}
            };
        });

        await expect(ToolPluginManager.fromEntryFiles({test: TEST_PLUGIN_FILE}))
            .rejects.toThrow(new InvalidPluginError("Plugin test does not have a description"));
    });

    test("test invalid plugin schema - null description", async () => {
        jest.mock(TEST_PLUGIN_FILE, () => {
            return {
                schema: {
                    description: ''
                }
            };
        });

        await expect(ToolPluginManager.fromEntryFiles({test: TEST_PLUGIN_FILE}))
            .rejects.toThrow(new InvalidPluginError("Plugin test does not have a description"));
    });

    test("test invalid plugin schema - argument invalid type", async () => {
        jest.mock(TEST_PLUGIN_FILE, () => {
            return {
                schema: {
                    description: 'this is a description',
                    arguments: {
                        test: {
                            type: 'aasdad',
                            description: 'Test argument',
                        }
                    }
                }
            };
        });

        await expect(ToolPluginManager.fromEntryFiles({test: TEST_PLUGIN_FILE}))
            .rejects.toThrow(new InvalidPluginError("Plugin test, argument test must have a type as 'string', 'number' or 'boolean'"));
    });

    test("test invalid plugin schema - argument without description", async () => {
        jest.mock(TEST_PLUGIN_FILE, () => {
            return {
                schema: {
                    description: 'this is a description',
                    arguments: {
                        test: {
                            type: 'string',
                        }
                    }
                }
            };
        });

        await expect(ToolPluginManager.fromEntryFiles({test: TEST_PLUGIN_FILE}))
            .rejects.toThrow(new InvalidPluginError("Plugin test, argument test does not have a description"));
    });
});