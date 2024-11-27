#/usr/bin/env bash

PROMPT_DIR="$HOME/.prompt"
BIN_PROMPT_DIR="$PROMPT_DIR/bin/"

mkdir -p $PROMPT_DIR
mkdir -p $BIN_PROMPT_DIR

cp config.yaml $PROMPT_DIR
cp dist/prompt.cjs $BIN_PROMPT_DIR
