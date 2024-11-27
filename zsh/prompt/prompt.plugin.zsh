#/usr/bin/env bash

export PROMPT_MODEL_ID=""
export PROMPT_CHAT_ID=""

change_prompt() {
  export PROMPT_CHAT_ID="$1"
  export PROMPT_MODEL_ID="$2"
}

function end_prompt() {
  unset PROMPT_CHAT_ID
  unset PROMPT_MODEL_ID
}

function prompt_model_and_chat() {
  if [ "$PROMPT_CHAT_ID" ]; then
    local MODEL="${PROMPT_MODEL_ID:-default}"
    local AGENT_PROMPT=""
    AGENT_PROMPT+="%{$fg_bold[blue]%}chat:(%{$reset_color%}"
    AGENT_PROMPT+="%{$fg_bold[red]%}$PROMPT_CHAT_ID%{$reset_color%}"
    AGENT_PROMPT+="%{$fg_bold[blue]%}:%{$reset_color%}"
    AGENT_PROMPT+="%{$fg_bold[red]%}$MODEL%{$reset_color%}"
    AGENT_PROMPT+="%{$fg_bold[blue]%})%{$reset_color%}"
    echo -n "$AGENT_PROMPT"
  fi
}
