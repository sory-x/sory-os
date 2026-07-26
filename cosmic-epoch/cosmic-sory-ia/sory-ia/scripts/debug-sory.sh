#!/bin/bash

# Set "chatgpt.cliExecutable": "/Users/<USERNAME>/code/sory/scripts/debug-sory.sh" in VSCode settings to always get the 
# latest sory-rs binary when debugging sory Extension.


set -euo pipefail

sory_RS_DIR=$(realpath "$(dirname "$0")/../sory-rs")
(cd "$sory_RS_DIR" && cargo run --quiet --bin sory -- "$@")