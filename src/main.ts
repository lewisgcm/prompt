import os from "os";
import path from "path";
import {program} from "commander";
import {loadConfigFile} from "./config";
import {ToolPluginManager} from "./tool";
import {resolveModel} from "./model";
import {pluck} from "./util";

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
    .requiredOption('-t --text-prompt <prompt>', 'textual prompt to send to the agent');

program.parse();

const options = program.opts();

(async () => {
    const config = await loadConfigFile(options.configFile);
    const modelName = options.model || config["default-model"];
    const modelConfig = (config?.models ?? {})[modelName];
    if (!modelConfig) {
        program.error(`could not find configuration for model '${modelName}'`);
    }

    const toolPluginManager = ToolPluginManager.fromEntryFiles(pluck(config["tool-plugins"] || {}, ...(modelConfig.tools || [])));
    const model = resolveModel(modelName, modelConfig, toolPluginManager);
    if (!model) {
        program.error(`failed to resolve model for the provider '${modelConfig.provider}'`);
    }

    const res = await model.send({type: 'text', value: options.textPrompt});
    console.log(res);
})();