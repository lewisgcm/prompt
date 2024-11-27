#/usr/bin/env bash

PROMPT_MODEL_ID=""
PROMPT_CHAT_ID=""

agent() {
  PROMPT_CHAT_ID="$1"
  PROMPT_MODEL_ID="$2"
}

function endprompt() {
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
