export type Prompt = { type: 'text', value: string } |
    { type: 'image', value: Uint8Array, format: 'png' | 'jpeg' | 'gif' | 'webp' } |
    {
        type: 'document',
        value: Uint8Array,
        format: 'pdf' | 'csv' | 'doc' | 'docx' | 'xls' | 'xlsx' | 'html' | 'txt' | 'md',
        name: string
    };

export type PromptResponse = { type: 'text', value: string };

export interface ModelPlugin {
    prompt(prompt: Prompt): Promise<PromptResponse[]>;

    configure(configuration: Configuration): Promise<void>;

    configuration(): ConfigurationStep[];

    test(): Promise<void>
}

export type ConfigurationType = string | number | boolean | null;

export interface Configuration {
    [key: string]: ConfigurationType
}

export interface ConfigurationInput {
    displayName: string;
    type: 'select' | 'bool' | 'integer' | 'float' | 'string';
    options?: string[];
    required: boolean;
}

export interface ConfigurationStep {
    input: (context: Configuration) => Promise<{ [key: string]: ConfigurationInput }> | {
        [key: string]: ConfigurationInput
    };
}