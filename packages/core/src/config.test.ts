import {describe, expect, test} from "@jest/globals";
import {InvalidConfigurationError, loadConfigFile, writeConfigFile} from "./config";
import path from "path";
import fs from "node:fs/promises";

describe("config tests", () => {
    test("test config file doesnt exist", async () => {
        await expect(async () => await loadConfigFile("some-rando-non-existant-file-lol")).rejects.toThrow(InvalidConfigurationError);
    });

    test("test basic read write config file", async () => {
        const directory = path.join(__dirname, "..", "test", "test_output");

        await writeConfigFile(directory, "test.config.yaml", {"default-model": "bleep"});
        const config = await loadConfigFile(directory + path.sep + "test.config.yaml");

        expect(config).toStrictEqual({"default-model": "bleep"});

        await fs.rm(directory + path.sep + "test.config.yaml");
    });
});