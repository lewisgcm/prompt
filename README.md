# Prompt - AI CLI
## Installation

### CLI


### oh-my-zsh
Prompt includes a plugin for oh-my-zsh which shows the currently selected model, and conversation in the zsh prompt string. This removes the need to pass the same arguments on each call to the prompt cli.

You can install this by first copying the `zsh/prompt` folder from this repository into your oh-my-zsh custom plugin directory with the following command:
```bash
cp -r zsh/prompt ~/.oh-my-zsh/custom/plugins
```

Next, add the prompt plugin into your `~/.zshrc` file by using `vim ~/.zshrc` and then adding `prompt` to the plugins list. Below is an example configuration.
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

Finally, include the current chat and model if set into your zsh prompt by editing your current zsh theme (this can be found in the `~/.zshrc` file. Below is an example from the default `vim ~/.oh-my-zsh/themes/robbyrussell.zsh-theme` theme.
```bash
PROMPT='${ret_status} %{$fg[cyan]%}%c%{$reset_color%} $(prompt_model_and_chat) $(git_prompt_info)'
```