import {describe, expect, test} from "@jest/globals";
import {mapBedrockToolUseBlockToToolUseResult} from "./helpers";

describe('helpers tests', () => {
    test("tool result mapping success", async () => {
        const result = await mapBedrockToolUseBlockToToolUseResult(
            {
                input: {
                    argument: "value"
                },
                name: "name",
                toolUseId: "id"
            },
            async () => Promise.resolve("test"),
        );

        expect(result).toStrictEqual({
            toolUseId: "id",
            status: "success",
            content: [
                {
                    json: "test"
                }
            ]
        });
    });

    test("tool result mapping error", async () => {
        const result = await mapBedrockToolUseBlockToToolUseResult(
            {
                input: {
                    argument: "value"
                },
                name: "name",
                toolUseId: "id"
            },
            async () => {
                throw new Error("failed to do stuff")
            },
        );

        expect(result).toStrictEqual({
            toolUseId: "id",
            status: "error",
            content: [
                {
                    text: "Error: failed to do stuff"
                }
            ]
        });
    });
});