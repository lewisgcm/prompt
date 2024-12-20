import * as fs from "node:fs/promises";
import path from "path";
import {Message} from "@aws-sdk/client-bedrock-runtime";

import {ConversationStore} from "../model";
import {fileExists} from "../../util";

export class BedrockStore extends ConversationStore<Message> {
    messages: Message[] = [];

    async getHistory(): Promise<Message[]> {
        if (this.messages.length > 0) {
            return this.messages;
        }

        if (!await fileExists(this.filePath())) {
            return this.messages;
        }

        const messages = [];
        const file = await fs.open(this.filePath(), 'r');
        for await (const line of file.readLines()) {
            messages.push(JSON.parse(line));
        }
        this.messages = messages;

        return this.messages;
    }

    setHistory(history: Message[]): void {
        this.messages = history;
    }

    async flush(): Promise<void> {
        await fs.mkdir(this.directory, {recursive: true});

        const file = await fs.open(this.filePath(), 'w');
        for (const message of this.messages) {
            await file.write(JSON.stringify(message) + '\n');
        }
    }

    private filePath(): string {
        return path.join(this.directory, this.conversationId);
    }
}