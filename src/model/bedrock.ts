import {Model, Prompt} from "./model";
import {ToolPluginManager} from "../tool";
import {ModelConfig} from "../config";
import {BedrockRuntimeClient, ConverseCommand, Tool, ToolConfiguration} from "@aws-sdk/client-bedrock-runtime";

export interface BedrockConfig extends ModelConfig {
    provider: 'bedrock',
    settings: {
        region?: string;
        'model-id': string;
    };
    tools?: string[];
}

export class BedrockModel extends Model {
    constructor(
        private readonly _toolPluginManager: ToolPluginManager,
        private readonly _config: BedrockConfig,
        private readonly _bedrockClient: BedrockRuntimeClient = new BedrockRuntimeClient({region: _config.settings.region})
    ) {
        super();
    }

    async send(prompt: Prompt): Promise<string> {
        const toolConfig = buildBedrockToolConfig(this._toolPluginManager)
        const response = await this._bedrockClient.send(new ConverseCommand({
            toolConfig: toolConfig,
            modelId: this._config.settings["model-id"],
            messages: [
                {
                    role: 'user',
                    content: [
                        {
                            text: prompt.value.toString()
                        }
                    ]
                }
            ],
        }));

        console.log(response);

        return prompt.value.toString();
    }
}

function buildBedrockToolConfig(toolPluginManager: ToolPluginManager): ToolConfiguration | undefined {
    const config = toolPluginManager.getAvailableTools();
    const tools = Object.entries(config).map(([key, value]) => {
        const requiredArguments = Object.entries(value.schema.arguments || {})
            .filter(([key, value]) => value.required)
            .map(([key, value]) => key);

        const properties = Object.entries(value.schema.arguments || {}).reduce((dict, [key, value]) => {
            return {
                ...dict,
                [key]: {
                    type: value.type,
                    description: value.description,
                },
            };
        }, {});

        return {
            toolSpec: {
                name: key,
                description: value.schema.description,
                inputSchema: {
                    json: {
                        type: 'object',
                        properties: properties,
                        required: requiredArguments
                    }
                }
            }
        } as Tool;
    });

    if (tools.length == 0) return undefined;

    return {
        tools: tools,
    };
}