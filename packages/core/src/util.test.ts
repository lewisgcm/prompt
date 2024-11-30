import {describe, expect, test} from "@jest/globals";
import {fileExists, pluck} from "./util";
import path from "path";

describe("utility function tests", () => {
    test("pluck empty", () => {
        const plucked = pluck({});
        expect(plucked).toEqual({});
    });

    test("pluck partial", () => {
        const plucked = pluck({key: 'value', test: 'sample'}, 'key');
        expect(plucked).toStrictEqual({key: 'value'});
    });

    test("file doesnt exist", async () => {
        const exists = await fileExists("asdasdasdasd");
        expect(exists).toBe(false);
    });

    test("file does exist", async () => {
        const exists = await fileExists(path.join(__dirname, "util.test.ts"));
        expect(exists).toBe(true);
    });
});