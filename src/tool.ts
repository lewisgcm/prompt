export interface ToolSchema {
    description: string;
    arguments?: {
        [key: string]: {
            type: 'string' | 'number' | 'boolean';
            description: string;
            required?: boolean;
        };
    };
}

export class InvalidPluginError extends Error {
    constructor(message = "") {
        super(message);
    }
}

export class ToolPluginManager {
    private constructor(private readonly _plugins: {
        [key: string]: { schema: ToolSchema, tool: (any: any) => any; }
    }) {
    }

    getAvailableTools(): { [key: string]: { schema: ToolSchema; } } {
        return this._plugins;
    }

    invokeTool(toolName: string, toolArguments: any): any {
        if (this._plugins.hasOwnProperty(toolName)) {
            return this._plugins[toolName].tool(toolArguments);
        }

        return undefined;
    }

    static fromEntryFiles(entryFiles: { [key: string]: string }): ToolPluginManager {
        const plugins = Object.entries(entryFiles).reduce((dict, [key, value]) => {
            const module = require(value);
            return {
                ...dict,
                [key]: {
                    schema: ToolPluginManager.validatePluginSchema(key, module.schema),
                    tool: module.tool as (any: any) => (any),
                }
            }
        }, {});

        return new ToolPluginManager(plugins);
    }

    private static validatePluginSchema(name: string, schema: ToolSchema): ToolSchema {
        if (!/\S/.test(schema.description)) {
            throw new InvalidPluginError(`Plugin ${name} does not have a description`);
        }

        Object.entries(schema.arguments || {}).forEach(([key, value]) => {
            if (!['string', 'number', 'boolean'].includes(value.type)) {
                throw new InvalidPluginError(`Plugin ${name}, argument ${key} must have a type as 'string', 'number' or 'boolean'`);
            }

            if (!/\S/.test(value.description)) {
                throw new InvalidPluginError(`Plugin ${name}, argument ${key} does not have a description`);
            }
        });

        return schema;
    }
}