# Prompt - AI CLI

Prompt cli enables you to run prompts against AWS bedrock models directly from your terminal.

## Getting started

### Installation

First download this repository and run the `npm install` command, then `npm run build` and finally `npm link`. Then run the `prompt setup` to
configure a default model. Use the `prompt -h` to get a list of available commands and settings.

**NOTE**: You need to have AWS credentials to run prompt, you typically get those by running `aws sso login` if your
account is configured for SSO.

### oh-my-zsh

Prompt includes a plugin for oh-my-zsh which shows the currently selected model, and conversation in the zsh prompt
string. This removes the need to pass the same arguments on each call to the prompt cli.

You can install this by first copying the `zsh/prompt` folder from this repository into your oh-my-zsh custom plugin
directory with the following command:

```bash
cp -r zsh/prompt ~/.oh-my-zsh/custom/plugins
```

Next, add the prompt plugin into your `~/.zshrc` file by using `vim ~/.zshrc` and then adding `prompt` to the plugins
list. Below is an example configuration.

```bash
# Which plugins would you like to load? (plugins can be found in ~/.oh-my-zsh/plugins/*)
# Custom plugins may be added to ~/.oh-my-zsh/custom/plugins/
# Example format: plugins=(rails git textmate ruby lighthouse)
# Add wisely, as too many plugins slow down shell startup.
plugins=(
  git
  prompt
)
```

Finally, include the current chat and model if set into your zsh prompt by editing your current zsh theme (this can be
found in the `~/.zshrc` file. Below is an example from the default `vim ~/.oh-my-zsh/themes/robbyrussell.zsh-theme`
theme.

```bash
PROMPT='${ret_status} %{$fg[cyan]%}%c%{$reset_color%} $(prompt_model_and_chat) $(git_prompt_info)'
```

## Usage

* ``

## Plugins

Prompt provides support for model function calling through plugins. Plugins are javascript modules exporting the
`schema` constant and `tool` method. They are loaded at runtime when interacting with AI agents.
You configure models by running the `prompt setup` command and then selecting the 'Add plugin' option. There are two
plugin settings:

* **name**: Your name for the plugin, which is used as an identifier in configuration files when enabling for specific
  models.
* **location**: The location of the plugins main entry file on your filesystem. Typically located within the
  `.prompt/plugins` directory.

An example plugin is below:

```javascript
// The tool will be called with the arguments defined in the schema.
export function tool({sign}) {
    return {song: "Elemental Hotel", artist: "8 Storey Hike", sign: sign};
}

export const schema = {
    // Describe the plugin, and what it does. 
    description: 'Get the most popular song played on a radio station.',
    arguments: {
        sign: {
            type: 'string',
            // Be as verbose as possible when describing arguments. Provide example values. 
            description: 'The call sign for the radio station for which you want the most popular song. Example calls signs are WZPZ and WKRP.',
            required: true, // Boolean which can be true or false
        }
    }
};
```