export type Prompt = { type: 'text', value: string } |
    { type: 'image', value: Int8Array, format: string } |
    { type: 'document', value: Int8Array, format: string };

export abstract class Model {
    abstract send(prompt: Prompt): Promise<string>;
}