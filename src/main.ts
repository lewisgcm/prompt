import os from "os";
import path from "path";
import {program} from "commander";
import {loadConfigFile} from "./config";
import {ToolPluginManager} from "./tool";
import {resolveConversationStore, resolveModel} from "./model";
import {pluck} from "./util";
import {Prompt} from "./model/model";
import * as fs from "node:fs";
import chalk from "chalk";
import ora from "ora";

const DEFAULT_HOME_DIRECTORY = os.homedir() + path.sep + '.prompt';

program
    .name('prompt')
    .description('CLI to interact with generative AI agents')
    .version('0.1.0');

program
    .requiredOption('-f --config-file <file>', 'YAML configuration file for prompt')
    .option('-d --home-dir <directory>', 'directory used to store chat history', DEFAULT_HOME_DIRECTORY)
    .requiredOption('-c --conversation <directory>', 'unique identifier to persist prompts for the conversation')
    .option('-m --model <model>', 'model name from the YAML file, to use for the prompt')
    .option('-it --input-text <prompt>', 'textual prompt to send to the agent')
    .option('-id --input-document <file>', 'document file to send to the agent')
    .option('-ii --input-image <file>', 'image file to send to the agent');

program.parse();

const options = program.opts();

(async () => {
    const config = await loadConfigFile(options.configFile);
    const modelName = options.model || config["default-model"];
    const modelConfig = (config?.models ?? {})[modelName];
    if (!modelConfig) {
        program.error(`could not find configuration for model '${modelName}'`);
    }

    const conversationStore = resolveConversationStore(modelConfig, options.homeDir, options.conversation);
    if (!conversationStore) {
        program.error(`could not load conversation store for '${options.conversation}'`);
    }

    const toolPluginManager = ToolPluginManager.fromEntryFiles(pluck(config["tool-plugins"] || {}, ...(modelConfig.tools || [])));
    const model = resolveModel(modelConfig, toolPluginManager, conversationStore);
    if (!model) {
        program.error(`failed to resolve model for the provider '${modelConfig.provider}'`);
    }

    const prompt = buildPrompt(options);
    if (!prompt) {
        program.error(`please specify either a text, image or document input as a prompt`);
    }

    const spinner = ora(`Awaiting response from ${chalk.red(modelName)}`).start();
    model.send(prompt).subscribe({
        next: (response) => {
            spinner.stop();

            switch (response.event) {
                case "max_tokens":
                    console.log(chalk.red(`reached the max tokens of the model [${response.usage.totalTokens}]`));
                    break;
                case "response":
                    response.content.forEach((c) => {
                        console.log(c.value)
                    });
                    spinner.start();
                    break;
                case "end":
                    console.log('');
                    console.log(`Total tokens: [${response.usage.totalTokens}], Input tokens: [${response.usage.inputTokens}], Output tokens: [${response.usage.outputTokens}]`);
                    break;
            }
        },
        complete: async () => {
            await conversationStore.flush();
        },
        error: (err) => {
            spinner.stop();
            console.log(err);
        }
    });
})();

// TODO: Add validation, and extension checking (with types). Otherwise it works.
function buildPrompt(options: { inputText?: string, inputDocument?: string, inputImage?: string }): Prompt | undefined {
    if (options.inputText) {
        return {type: 'text', value: options.inputText};
    } else if (options.inputDocument) {
        const data = fs.readFileSync(options.inputDocument);
        const ext = path.extname(options.inputDocument).substring(1);
        return {type: 'document', value: data, format: ext, name: options.inputDocument};
    } else if (options.inputImage) {
        const data = fs.readFileSync(options.inputImage);
        const ext = path.extname(options.inputImage).substring(1);
        return {type: 'image', value: data, format: ext};
    }

    return undefined;
}