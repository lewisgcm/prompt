import yaml from "js-yaml";
import {promises} from "node:fs";

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