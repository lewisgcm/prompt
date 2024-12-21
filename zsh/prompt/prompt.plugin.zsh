#/usr/bin/env bash

export PROMPT_MODEL_ID=""
export PROMPT_CHAT_ID=""

prompt_change() {
  conv_file=$(mktemp /tmp/prompt-conv.XXXXXX)
  model_file=$(mktemp /tmp/prompt-model.XXXXXX)

  prompt select-model --conversation-out-file "$conv_file" --model-out-file "$model_file"

  export PROMPT_CHAT_ID=`cat "$conv_file"`
  export PROMPT_MODEL_ID=`cat "$model_file"`
}

function prompt_end() {
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
