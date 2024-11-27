#/usr/bin/env bash

PROMPT_DIR="$HOME/.prompt"
BIN_PROMPT_DIR="$PROMPT_DIR/bin/"

CONFIG="$HOME/.prompt/config.yaml"
MODEL_ID=""
CHAT_ID=""

agent() {
  CHAT_ID="$1"
  MODEL_ID="$2"
}

prompt() {
    if [ "$MODEL_ID" ]; then
      node $BIN_PROMPT_DIR/prompt.cjs --config-file $CONFIG -c $CHAT_ID -m $MODEL_ID
    else
      node $BIN_PROMPT_DIR/prompt.cjs --config-file $CONFIG -c $CHAT_ID
    fi
}

endprompt() {
  unset CHAT_ID
  unset MODEL_ID
}

export -f prompt
export -f endprompt

setopt prompt_subst

function include_prompt_agent() {
  if [ "$CHAT_ID" ]; then
    local MODEL="${MODEL_ID:-default}"
    local AGENT_PROMPT=' '
    AGENT_PROMPT+="%{$fg_bold[blue]%}agent:(%{$reset_color%}"
    AGENT_PROMPT+="%{$fg_bold[red]%}$CHAT_ID%{$reset_color%}"
    AGENT_PROMPT+="%{$fg_bold[yellow]%} @ %{$reset_color%}"
    AGENT_PROMPT+="%{$fg_bold[red]%}$MODEL%{$reset_color%}"
    AGENT_PROMPT+="%{$fg_bold[blue]%})%{$reset_color%}"
    PROMPT="%{$fg_bold[cyan]%}%1~%{$reset_color%}$AGENT_PROMPT "
  else
    PROMPT=' '
  fi
}

typeset -a precmd_functions
precmd_functions+=(include_prompt_agent)