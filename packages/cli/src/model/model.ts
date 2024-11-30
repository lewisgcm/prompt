import {Subject} from "rxjs";

export interface Usage {
    totalTokens: number;
    inputTokens: number;
    outputTokens: number;
    latency: number;
}

export type Prompt = { type: 'text', value: string } |
    { type: 'image', value: Uint8Array, format: string } |
    { type: 'document', value: Uint8Array, format: string, name: string };

export type ModelResponseMessageContent = { type: 'text', value: string };

export type ModelResponseEvent = { event: 'end', usage: Usage }
    | { event: 'response', content: ModelResponseMessageContent[] }
    | { event: 'max_tokens', usage: Usage };

export abstract class Model {
    abstract send(prompt: Prompt): Subject<ModelResponseEvent>;
}

export abstract class ConversationStore<T> {
    constructor(
        protected readonly homeDir: string,
        protected readonly conversationId: string) {
    }

    abstract getHistory(): Promise<T[]>

    abstract setHistory(history: T[]): void

    abstract flush(): Promise<void>
}