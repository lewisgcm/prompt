import yaml from "js-yaml";
import {promises} from "node:fs";
import fs from "node:fs/promises";

export interface ModelConfig {
    provider: string,
    settings?: {
        [key: string]: any;
    };
    tools?: string[];
}

export interface Config {
    'default-model'?: string;
    models?: {
        [key: string]: ModelConfig
    };
    'tool-plugins'?: {
        [key: string]: string;
    };
}

export class InvalidConfigurationError extends Error {
    constructor(message = "") {
        super(message);
    }
}

export async function loadConfigFile(path: string): Promise<Config> {
    try {
        const rawConfig = await promises.readFile(path, "utf8");
        return yaml.load(rawConfig.toString()) as Config;
    } catch (e) {
        throw new InvalidConfigurationError("failed to parse config file");
    }
}

export async function writeConfigFile(path: string, filename: string, config: Config): Promise<void> {
    try {
        await fs.mkdir(path, {recursive: true});
        const rawConfig = yaml.dump(config);
        await promises.writeFile(filename, rawConfig);
    } catch (e) {
        throw new Error("failed to write config file");
    }
}