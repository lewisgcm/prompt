import {Subject} from "rxjs";

export type Prompt = { type: 'text', value: string } |
    { type: 'image', value: Uint8Array, format: string } |
    { type: 'document', value: Uint8Array, format: string, name: string };

export abstract class Model {
    abstract send(prompt: Prompt): Subject<string>;
}